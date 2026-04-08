use sqlx::postgres::{PgPool, PgPoolOptions};


#[allow(unused)]
pub async fn criar_pool() -> Result<PgPool, sqlx::Error> {
    if let Err(e) = dotenvy::from_filename("database.secrets.env") {
        tracing::error!("⚠️ Aviso: Não foi possível carregar database.secrets.env: {}", e);
        tracing::error!("   Certifique-se de que a variável DATABASE_URL está definida no ambiente.");
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


#[allow(unused)]
pub async fn aplicar_migrations(pool: &PgPool) -> Result<(), String> {
    // let mode = std::env::var("MODE").unwrap_or_default();
    // let is_dev = mode.eq_ignore_ascii_case("development");

    // if is_dev {
    //     tracing::info!("🧹 MODE=DEVELOPMENT — limpando banco de dados antes de migrar");
    //     drop_all_tables(pool).await?;
    // }

    tracing::info!("📦 Aplicando migrações...");
    run_migrations(pool).await?;
    tracing::info!("✅ Migrações aplicadas com sucesso");

    Ok(())
}

/// Executa os arquivos de migração em ordem, aplicando apenas as pendentes
async fn run_migrations(pool: &PgPool) -> Result<(), String> {
    // Cria a tabela de controle de migrações se não existir
    create_migration_table(pool).await?;

    // Busca a última migração aplicada
    let last_applied = get_last_applied_migration(pool).await?;
    tracing::info!("📋 Última migração aplicada: {:?}", last_applied);

    let migration_files = [
        "0001_criar_tabelas.sql",
        "0002_add_promocao_escopo.sql",
        "0003_add_criado_por_lojas.sql",
        "0004_add_pizza_mode_categorias.sql",
        "0005_add_entregador_uuid_pedidos.sql",
    ];

    for filename in &migration_files {

        let version = filename
            .split('_')
            .next()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or_default();

        if let Some(last) = last_applied {
            if version <= last {
                tracing::info!("   ⏭️  {} (já aplicada)", filename);
                continue;
            }
        }

        let migration_path = format!("migrations/{}", filename);
        let sql = match std::fs::read_to_string(&migration_path) {
            Ok(content) => content,
            Err(_) => match std::fs::read_to_string(format!("src/../{}", migration_path)) {
                Ok(content) => content,
                Err(e) => return Err(format!("Não foi possível ler o arquivo de migração {}: {}", migration_path, e)),
            },
        };

        let statements = split_sql_statements(&sql)?;

        for (i, stmt) in statements.iter().enumerate() {
            sqlx::query(stmt)
                .execute(pool)
                .await
                .map_err(|e| format!("Falha no statement #{} em {}: {}", i + 1, migration_path, e))?;
        }

        // Registra a migração como aplicada
        record_migration(pool, version, filename).await?;

        tracing::info!("   ✅ {} -> {} statements executados", migration_path, statements.len());
    }

    Ok(())
}

/// Cria a tabela de controle de versões de migração
async fn create_migration_table(pool: &PgPool) -> Result<(), String> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            filename TEXT NOT NULL,
            applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )"
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Falha ao criar tabela schema_migrations: {}", e))?;

    tracing::info!("📊 Tabela schema_migrations verificada");
    Ok(())
}

/// Retorna a versão da última migração aplicada
async fn get_last_applied_migration(pool: &PgPool) -> Result<Option<u32>, String> {
    let stmt = "SELECT MAX(version) FROM schema_migrations";
    let result = sqlx::query_scalar::<_, Option<i32>>(stmt)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Falha ao buscar última migração: {}", e))?;

    Ok(result.map(|v| v as u32))
}

/// Registra uma migração como aplicada
async fn record_migration(pool: &PgPool, version: u32, filename: &str) -> Result<(), String> {
    let stmt = "INSERT INTO schema_migrations (version, filename) VALUES ($1, $2)";
    sqlx::query(stmt)
        .bind(version as i32)
        .bind(filename)
        .execute(pool)
        .await
        .map_err(|e| format!("Falha ao registrar migração {}: {}", version, e))?;

    tracing::info!("   📝 Migração {} registrada em schema_migrations", version);
    Ok(())
}


#[allow(unused)]
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


#[allow(unused)]
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
