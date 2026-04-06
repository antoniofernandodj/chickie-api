use sea_orm::{Database, DatabaseConnection, DbErr, ConnectionTrait};

pub async fn criar_conexao() -> Result<DatabaseConnection, DbErr> {
    if let Err(e) = dotenvy::from_filename("database.secrets.env") {
        tracing::error!("⚠️ Aviso: Não foi possível carregar database.secrets.env: {}", e);
        tracing::error!("   Certifique-se de que a variável DATABASE_URL está definida no ambiente.");
    }

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL não encontrado");

    let db = Database::connect(&database_url).await?;

    tracing::info!("✅ Conexão com banco estabelecido");
    Ok(db)
}

/// Aplica as migrações no banco de dados.
/// Se `MODE == DEVELOPMENT`, dropa todas as tabelas antes de reaplicar.
pub async fn aplicar_migrations(db: &DatabaseConnection) -> Result<(), String> {
    let mode = std::env::var("MODE").unwrap_or_default();
    let is_dev = mode.eq_ignore_ascii_case("development");

    if is_dev {
        tracing::info!("🧹 MODE=DEVELOPMENT — limpando banco de dados antes de migrar");
        drop_all_tables(db).await?;
    }

    tracing::info!("📦 Aplicando migrações...");
    run_migrations(db).await?;
    tracing::info!("✅ Migrações aplicadas com sucesso");

    Ok(())
}

/// Executa os arquivos de migração em ordem, dividindo em statements individuais
async fn run_migrations(db: &DatabaseConnection) -> Result<(), String> {
    let migration_files: [&str; 3] = [
        "0001_criar_tabelas.sql",
        "0002_add_promocao_escopo.sql",
        "0003_add_criado_por_lojas.sql"
    ];

    for migration_path in
        &migration_files.map(|f: &str| format!("migrations/{}", f))
    {
        let sql = match std::fs::read_to_string(migration_path) {
            Ok(content) => content,
            Err(_) => match std::fs::read_to_string(format!("src/../{}", migration_path)) {
                Ok(content) => content,
                Err(e) => return Err(format!("Não foi possível ler o arquivo de migração {}: {}", migration_path, e)),
            },
        };

        let statements = split_sql_statements(&sql)?;

        for (i, stmt) in statements.iter().enumerate() {
            db.execute(sea_orm::Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                stmt.to_string(),
            ))
            .await
            .map_err(|e| format!("Falha no statement #{} em {}: {}", i + 1, migration_path, e))?;
        }

        tracing::info!("   {} -> {} statements executados", migration_path, statements.len());
    }

    Ok(())
}

/// Divide SQL em statements individuais, respeutando blocos $$ e ignorando comentários
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
async fn drop_all_tables(db: &DatabaseConnection) -> Result<(), String> {
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

    db.execute(sea_orm::Statement::from_string(
        sea_orm::DatabaseBackend::Postgres,
        drop_sql.to_string(),
    ))
    .await
    .map_err(|e| format!("Falha ao dropar tabelas: {}", e))?;

    tracing::info!("🗑️ Todas as tabelas foram removidas");
    Ok(())
}
