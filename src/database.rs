use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn criar_pool() -> Result<PgPool, sqlx::Error> {
    if let Err(e) = dotenvy::from_filename("database.secrets.env") {
        eprintln!("⚠️ Aviso: Não foi possível carregar database.secrets.env: {}", e);
        eprintln!("   Certifique-se de que a variável DATABASE_URL está definida no ambiente.");
    }

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL não encontrado");


    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(std::time::Duration::from_secs(600))
        .max_lifetime(std::time::Duration::from_secs(1800))
        .after_connect(|conn, _meta| Box::pin(async move {
            sqlx::query("SET timezone = 'America/Sao_Paulo'")
                .execute(conn)
                .await?;
            Ok(())
        }))
        .connect(&database_url)
        .await?;

    Ok(pool)
}

/// Aplica as migrações no banco de dados.
/// Se `MODE == DEVELOPMENT`, dropa todas as tabelas antes de reaplicar.
pub async fn aplicar_migrations(pool: &PgPool) -> Result<(), String> {
    let mode = std::env::var("MODE").unwrap_or_default();
    let is_dev = mode.eq_ignore_ascii_case("development");

    if is_dev {
        tracing::info!("🧹 MODE=DEVELOPMENT — limpando banco de dados antes de migrar");
        drop_all_tables(pool).await?;
    }

    tracing::info!("📦 Aplicando migrações...");
    run_migrations(pool).await?;
    tracing::info!("✅ Migrações aplicadas com sucesso");

    Ok(())
}

/// Executa o arquivo de migração completo, dividindo em statements individuais
async fn run_migrations(pool: &PgPool) -> Result<(), String> {
    let sql = std::fs::read_to_string("migrations/0001_criar_tabelas.sql")
        .or_else(|_| std::fs::read_to_string("src/../migrations/0001_criar_tabelas.sql"))
        .map_err(|e| format!("Não foi possível ler o arquivo de migração: {}", e))?;

    // Split into individual statements, respecting $$ blocks and comments
    let statements = split_sql_statements(&sql)?;

    for (i, stmt) in statements.iter().enumerate() {
        sqlx::query(stmt)
            .execute(pool)
            .await
            .map_err(|e| format!("Falha no statement #{}: {}", i + 1, e))?;
    }

    tracing::info!("   {} statements executados", statements.len());
    Ok(())
}

/// Divide SQL em statements individuais, respeitando blocos $$ e ignorando comentários
fn split_sql_statements(sql: &str) -> Result<Vec<String>, String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut in_dollar_quote = false;
    let mut lines = sql.lines();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        // Skip comment-only lines
        if trimmed.starts_with("--") {
            continue;
        }

        // Track $$ blocks
        if !in_dollar_quote {
            if trimmed.contains("$$") {
                in_dollar_quote = true;
            }
        } else {
            if trimmed.contains("$$") {
                in_dollar_quote = false;
            }
        }

        if trimmed.is_empty() {
            continue;
        }

        current.push_str(line);
        current.push('\n');

        // If we're inside a $$ block, don't split on ;
        if in_dollar_quote {
            continue;
        }

        // Check if this line ends with ; (statement boundary)
        if trimmed.ends_with(';') {
            let stmt = current.trim().to_string();
            if !stmt.is_empty() {
                statements.push(stmt);
            }
            current.clear();
        }
    }

    // Handle remaining content
    let remaining = current.trim().to_string();
    if !remaining.is_empty() {
        statements.push(remaining);
    }

    Ok(statements)
}

/// Dropa todas as tabelas do schema public usando CASCADE
async fn drop_all_tables(pool: &PgPool) -> Result<(), String> {
    // Gera e executa DROP CASCADE para todas as tabelas
    let drop_sql = "
        DO $$ DECLARE
            r RECORD;
        BEGIN
            FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP
                EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
            END LOOP;
        END $$;
    ";

    sqlx::query(drop_sql)
        .execute(pool)
        .await
        .map_err(|e| format!("Falha ao dropar tabelas: {}", e))?;

    tracing::info!("🗑️ Todas as tabelas foram removidas");
    Ok(())
}
