use sqlx::postgres::{PgPool, PgPoolOptions};
use std::path::Path;

pub async fn criar_pool() -> Result<PgPool, sqlx::Error> {
    let env_path = "database.secrets.env";
    if Path::new(env_path).exists() {
        if let Err(e) = dotenvy::from_filename(env_path) {
            tracing::warn!("⚠️ Falha ao carregar database.secrets.env: {}", e);
        }
    } else {
        tracing::debug!("📄 database.secrets.env não encontrado, usando variáveis de ambiente");
    }

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL não encontrado no ambiente");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(&database_url)
        .await?;

    Ok(pool)
}

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

async fn run_migrations(pool: &PgPool) -> Result<(), String> {
    create_migration_table(pool).await?;

    let last_applied = get_last_applied_migration(pool).await?;
    tracing::info!("📋 Última migração aplicada: {:?}", last_applied);

    let migration_files = [
        "0001_criar_tabelas.sql",
        "0002_add_promocao_escopo.sql",
        "0003_add_criado_por_lojas.sql",
        "0004_add_pizza_mode_categorias.sql",
        "0005_add_entregador_uuid_pedidos.sql",
        "0006_consolidar_pedidos_jsonb.sql",
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
            Err(_) => match std::fs::read_to_string(format!("../{}", migration_path)) {
                Ok(content) => content,
                Err(e) => return Err(format!("Não foi possível ler {}: {}", migration_path, e)),
            },
        };

        let statements = split_sql_statements(&sql)?;

        for (i, stmt) in statements.iter().enumerate() {
            sqlx::query(stmt)
                .execute(pool)
                .await
                .map_err(|e| format!("Falha no statement #{} em {}: {}", i + 1, migration_path, e))?;
        }

        record_migration(pool, version, filename).await?;
        tracing::info!("   ✅ {} -> {} statements executados", migration_path, statements.len());
    }

    Ok(())
}

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
    .map_err(|e| format!("Falha ao criar schema_migrations: {}", e))?;

    Ok(())
}

async fn get_last_applied_migration(pool: &PgPool) -> Result<Option<u32>, String> {
    let result = sqlx::query_scalar::<_, Option<i32>>("SELECT MAX(version) FROM schema_migrations")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Falha ao buscar migração: {}", e))?;

    Ok(result.map(|v| v as u32))
}

async fn record_migration(pool: &PgPool, version: u32, filename: &str) -> Result<(), String> {
    sqlx::query("INSERT INTO schema_migrations (version, filename) VALUES ($1, $2)")
        .bind(version as i32)
        .bind(filename)
        .execute(pool)
        .await
        .map_err(|e| format!("Falha ao registrar migração {}: {}", version, e))?;

    Ok(())
}

fn split_sql_statements(sql: &str) -> Result<Vec<String>, String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut in_dollar_quote = false;

    for line in sql.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("--") {
            continue;
        }

        if !in_dollar_quote && trimmed.contains("$$") {
            in_dollar_quote = true;
        } else if in_dollar_quote && trimmed.contains("$$") {
            in_dollar_quote = false;
        }

        if trimmed.is_empty() {
            continue;
        }

        current.push_str(line);
        current.push('\n');

        if in_dollar_quote {
            continue;
        }

        if trimmed.ends_with(';') {
            let stmt = current.trim().to_string();
            if !stmt.is_empty() {
                statements.push(stmt);
            }
            current.clear();
        }
    }

    let remaining = current.trim().to_string();
    if !remaining.is_empty() {
        statements.push(remaining);
    }

    Ok(statements)
}

async fn drop_all_tables(pool: &PgPool) -> Result<(), String> {
    sqlx::query(
        "DO $$ DECLARE
            r RECORD;
        BEGIN
            FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP
                EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
            END LOOP;
        END $$;"
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Falha ao dropar tabelas: {}", e))?;

    tracing::info!("🗑️ Todas as tabelas removidas");
    Ok(())
}
