use sqlx::postgres::{PgPool, PgPoolOptions};
use std::path::Path;

/// Creates a PostgreSQL connection pool
/// 
/// Loads `database.secrets.env` if it exists, then reads `DATABASE_URL` from environment.
/// Configures timezone to America/Sao_Paulo and sets connection pool parameters.
pub async fn criar_pool() -> Result<PgPool, sqlx::Error> {
    // Tenta carregar database.secrets.env apenas se existir
    let env_path = "database.secrets.env";
    if Path::new(env_path).exists() {
        if let Err(e) = dotenvy::from_filename(env_path) {
            tracing::warn!("⚠️ Falha ao carregar database.secrets.env: {}", e);
        }
    } else {
        tracing::debug!("📄 database.secrets.env não encontrado, usando variáveis de ambiente do sistema");
    }

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL não encontrado no ambiente");

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

/// Applies all pending migrations to the database
/// 
/// If MODE=development, drops all tables before applying migrations.
/// Discovers migration files dynamically from the `migrations/` directory.
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
    seed_categorias_globais(pool).await?;

    Ok(())
}

/// Seeds global categories (loja_uuid IS NULL) if they don't exist
async fn seed_categorias_globais(pool: &PgPool) -> Result<(), String> {
    let categorias = vec![
        ("Pizzas", "Pizzas de diversos sabores e tamanhos", true, false),
        ("Hambúrgueres", "Hambúrgueres artesanais e clássicos", false, false),
        ("Bebidas", "Refrigerantes, sucos, cervejas e águas", false, true),
        ("Sobremesas", "Doces, bolos e sobremesas variadas", false, false),
        ("Porções", "Batata frita, calabresa, frango a passarinho e mais", false, false),
        ("Massas", "Macarrão, lasanha e outras massas italianas", false, false),
        ("Saladas", "Saladas frescas e acompanhamentos saudáveis", false, false),
        ("Açaí", "Açaí na tigela com diversos complementos", false, false),
        ("Pastéis", "Pastéis fritos na hora com recheios variados", false, false),
        ("Quentinhas", "Refeições completas e caseiras para o seu dia a dia", false, false),
        ("Quentinhas Fitness", "Refeições balanceadas e saudáveis para manter o foco", false, false),
    ];

    for (nome, descricao, pizza_mode, drink_mode) in categorias {
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM categorias_produtos WHERE loja_uuid IS NULL AND nome = $1)")
            .bind(nome)
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Falha ao verificar categoria {}: {}", nome, e))?;

        if !exists {
            sqlx::query("INSERT INTO categorias_produtos (nome, descricao, pizza_mode, drink_mode, loja_uuid) VALUES ($1, $2, $3, $4, NULL)")
                .bind(nome)
                .bind(descricao)
                .bind(pizza_mode)
                .bind(drink_mode)
                .execute(pool)
                .await
                .map_err(|e| format!("Falha ao semear categoria {}: {}", nome, e))?;
            tracing::info!("   ✅ Categoria global criada: {}", nome);
        } else {
            tracing::debug!("   ⏭️  Categoria global já existe: {}", nome);
        }
    }

    Ok(())
}

/// Discovers and executes migration files in order, applying only pending ones
async fn run_migrations(pool: &PgPool) -> Result<(), String> {
    create_migration_table(pool).await?;

    let last_applied = get_last_applied_migration(pool).await?;
    tracing::info!("📋 Última migração aplicada: {:?}", last_applied);

    // Dynamically discover migration files from migrations/ directory
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
        tracing::info!("   ✅ {} -> {} statements executados", filename, statements.len());
    }

    Ok(())
}

/// Dynamically discovers migration files in the migrations/ directory
/// 
/// Returns a sorted vector of migration filenames matching the pattern `NNNN_*.sql`.
fn discover_migrations() -> Result<Vec<String>, String> {
    // Try multiple possible base paths for the migrations directory
    let possible_paths = [
        "migrations".to_string(),
        "../migrations".to_string(),
        "src/../migrations".to_string(),
        "../../migrations".to_string(),
    ];

    let migrations_dir = possible_paths
        .iter()
        .find(|p| Path::new(p).exists() && Path::new(p).is_dir())
        .ok_or("Diretório migrations/ não encontrado. Procurei em: migrations, ../migrations, src/../migrations, ../../migrations")?;

    let entries = std::fs::read_dir(migrations_dir)
        .map_err(|e| format!("Falha ao ler diretório migrations/: {}", e))?;

    let mut migrations: Vec<String> = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                // Only include .sql files that start with a version number (e.g., 0001_*.sql)
                if name.ends_with(".sql") && name.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                    Some(name)
                } else {
                    None
                }
            })
        })
        .collect();

    // Sort to ensure correct order
    migrations.sort();

    if migrations.is_empty() {
        tracing::warn!("⚠️ Nenhuma migração encontrada no diretório migrations/");
    } else {
        tracing::info!("📂 Migrações descobertas: {}", migrations.join(", "));
    }

    Ok(migrations)
}

/// Reads a migration file from the migrations/ directory
/// 
/// Tries multiple path resolutions to find the migration file.
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

/// Creates the schema_migrations table if it doesn't exist
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

/// Returns the version of the last applied migration
async fn get_last_applied_migration(pool: &PgPool) -> Result<Option<u32>, String> {
    let result = sqlx::query_scalar::<_, Option<i32>>("SELECT MAX(version) FROM schema_migrations")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Falha ao buscar última migração: {}", e))?;

    Ok(result.map(|v| v as u32))
}

/// Records a migration as applied
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

/// Splits SQL content into individual statements
/// 
/// Handles $$ dollar-quoted blocks (used in PostgreSQL functions/triggers)
/// and ignores comment-only lines.
fn split_sql_statements(sql: &str) -> Result<Vec<String>, String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut in_dollar_quote = false;

    for line in sql.lines() {
        let trimmed = line.trim();

        // Skip comment-only lines
        if trimmed.starts_with("--") {
            continue;
        }

        // Track $$ blocks
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

/// Drops all tables in the public schema
#[allow(unused)]
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

    tracing::info!("🗑️ Todas as tabelas foram removidas");
    Ok(())
}
