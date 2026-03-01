use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub async fn criar_pool() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .after_connect(|conn, _meta| Box::pin(async move {
            sqlx::query("PRAGMA foreign_keys = ON")
                .execute(conn)
                .await?;
            Ok(())
        }))
        .connect("sqlite://db.sqlite?mode=rwc")
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}