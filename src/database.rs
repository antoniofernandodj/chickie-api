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

/// Executa o arquivo de migração completo
async fn run_migrations(pool: &PgPool) -> Result<(), String> {
    // Tenta carregar do diretório do binário ou do cwd
    let sql = std::fs::read_to_string("migrations/0001_criar_tabelas.sql")
        .or_else(|_| std::fs::read_to_string("src/../migrations/0001_criar_tabelas.sql"))
        .map_err(|e| format!("Não foi possível ler o arquivo de migração: {}", e))?;

    sqlx::query(&sql)
        .execute(pool)
        .await
        .map_err(|e| format!("Falha ao aplicar migração: {}", e))?;

    Ok(())
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
