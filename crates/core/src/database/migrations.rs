use sqlx::PgPool;
use std::path::Path;

use super::seeds;

/// Applies all pending migrations and seeds to the database.
///
/// If `MODE=development`, drops all tables before migrating and runs the test data seed.
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

    tracing::info!("🌱 Semeando categorias globais...");
    seeds::seed_categorias_globais(pool).await?;

    if is_dev {
        tracing::info!("🌱 Semeando dados de teste...");
        seeds::seed_dados_teste(pool).await?;
    }

    Ok(())
}

async fn run_migrations(pool: &PgPool) -> Result<(), String> {
    create_migration_table(pool).await?;

    let last_applied = get_last_applied_migration(pool).await?;
    tracing::info!("📋 Última migração aplicada: {:?}", last_applied);

    let migration_files = discover_migrations()?;

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

        let sql = read_migration_file(filename)?;
        let statements = split_sql_statements(&sql)?;

        for (i, stmt) in statements.iter().enumerate() {
            sqlx::query(stmt)
                .execute(pool)
                .await
                .map_err(|e| format!("Falha no statement #{} em {}: {}", i + 1, filename, e))?;
        }

        record_migration(pool, version, filename).await?;
        tracing::info!(
            "   ✅ {} -> {} statements executados",
            filename,
            statements.len()
        );
    }

    Ok(())
}

/// Discovers migration files in the `migrations/` directory, sorted by version.
fn discover_migrations() -> Result<Vec<String>, String> {
    let possible_paths = [
        "migrations".to_string(),
        "../migrations".to_string(),
        "src/../migrations".to_string(),
        "../../migrations".to_string(),
    ];

    let migrations_dir = possible_paths
        .iter()
        .find(|p| Path::new(p).exists() && Path::new(p).is_dir())
        .ok_or(
            "Diretório migrations/ não encontrado. Procurei em: migrations, ../migrations, src/../migrations, ../../migrations",
        )?;

    let entries = std::fs::read_dir(migrations_dir)
        .map_err(|e| format!("Falha ao ler diretório migrations/: {}", e))?;

    let mut migrations: Vec<String> = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                if name.ends_with(".sql")
                    && name.chars().next().map_or(false, |c| c.is_ascii_digit())
                {
                    Some(name)
                } else {
                    None
                }
            })
        })
        .collect();

    migrations.sort();

    if migrations.is_empty() {
        tracing::warn!("⚠️ Nenhuma migração encontrada no diretório migrations/");
    } else {
        tracing::info!("📂 Migrações descobertas: {}", migrations.join(", "));
    }

    Ok(migrations)
}

fn read_migration_file(filename: &str) -> Result<String, String> {
    let possible_paths = [
        format!("migrations/{}", filename),
        format!("../migrations/{}", filename),
        format!("src/../migrations/{}", filename),
        format!("../../migrations/{}", filename),
    ];

    for path in &possible_paths {
        if let Ok(content) = std::fs::read_to_string(path) {
            return Ok(content);
        }
    }

    Err(format!(
        "Não foi possível ler o arquivo de migração {}. Procurei em: {}",
        filename,
        possible_paths.join(", ")
    ))
}

/// Splits SQL content into individual statements, handling `$$` dollar-quoted blocks.
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

async fn create_migration_table(pool: &PgPool) -> Result<(), String> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            filename TEXT NOT NULL,
            applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )",
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Falha ao criar tabela schema_migrations: {}", e))?;

    tracing::info!("📊 Tabela schema_migrations verificada");
    Ok(())
}

async fn get_last_applied_migration(pool: &PgPool) -> Result<Option<u32>, String> {
    let result =
        sqlx::query_scalar::<_, Option<i32>>("SELECT MAX(version) FROM schema_migrations")
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Falha ao buscar última migração: {}", e))?;

    Ok(result.map(|v| v as u32))
}

async fn record_migration(pool: &PgPool, version: u32, filename: &str) -> Result<(), String> {
    sqlx::query("INSERT INTO schema_migrations (version, filename) VALUES ($1, $2)")
        .bind(version as i32)
        .bind(filename)
        .execute(pool)
        .await
        .map_err(|e| format!("Falha ao registrar migração {}: {}", version, e))?;

    tracing::info!("   📝 Migração {} registrada em schema_migrations", version);
    Ok(())
}

async fn drop_all_tables(pool: &PgPool) -> Result<(), String> {
    sqlx::query(
        "DO $$ DECLARE
            r RECORD;
        BEGIN
            FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP
                EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
            END LOOP;
        END $$;",
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Falha ao dropar tabelas: {}", e))?;

    tracing::info!("🗑️ Todas as tabelas foram removidas");
    Ok(())
}
