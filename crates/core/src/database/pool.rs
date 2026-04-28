use sqlx::postgres::{PgPool, PgPoolOptions};
use std::path::Path;

/// Creates a PostgreSQL connection pool.
///
/// Loads `database.secrets.env` if it exists, then reads `DATABASE_URL` from environment.
/// Configures timezone to America/Sao_Paulo and sets connection pool parameters.
pub async fn criar_pool() -> Result<PgPool, sqlx::Error> {
    let env_path = "database.secrets.env";
    if Path::new(env_path).exists() {
        if let Err(e) = dotenvy::from_filename(env_path) {
            tracing::warn!("⚠️ Falha ao carregar database.secrets.env: {}", e);
        }
    } else {
        tracing::debug!(
            "📄 database.secrets.env não encontrado, usando variáveis de ambiente do sistema"
        );
    }

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL não encontrado no ambiente");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(std::time::Duration::from_secs(600))
        .max_lifetime(std::time::Duration::from_secs(1800))
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                sqlx::query("SET timezone = 'America/Sao_Paulo'")
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .connect(&database_url)
        .await?;

    Ok(pool)
}
