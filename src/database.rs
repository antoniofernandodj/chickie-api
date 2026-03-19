use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn criar_pool() -> Result<PgPool, sqlx::Error> {
    if let Err(e) = dotenvy::from_filename("database.secrets.env") {
        eprintln!("⚠️ Aviso: Não foi possível carregar database.secrets.env: {}", e);
        eprintln!("   Certifique-se de que a variável DATABASE_URL está definida no ambiente.");
    }

    // let postgres_user = std::env::var("POSTGRES_USER")
    //     .expect("POSTGRES_USER não encontrado");
    // let postgres_password = std::env::var("POSTGRES_PASSWORD")
    //     .expect("POSTGRES_PASSWORD não encontrado");
    // let postgres_db = std::env::var("POSTGRES_DB")
    //     .expect("POSTGRES_DB não encontrado");
    // let postgres_host = std::env::var("POSTGRES_HOST")
    //     .expect("POSTGRES_HOST não encontrado");
    // let postgres_port = std::env::var("POSTGRES_PORT")
    //     .expect("POSTGRES_PORT não encontrado");


    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL não encontrado");

    // let database_url = format!("postgres://{}:{}@{}:{}/{}",
    //     postgres_user,
    //     postgres_password,
    //     postgres_host,
    //     postgres_port,
    //     postgres_db,
    // );

    eprintln!("{}", database_url);

    let pool = PgPoolOptions::new()
        .max_connections(10)              // Conexões máximas no pool
        .min_connections(2)               // Conexões mínimas mantidas
        .acquire_timeout(std::time::Duration::from_secs(30))  // Timeout para adquirir conexão
        .idle_timeout(std::time::Duration::from_secs(600))    // Fecha conexões ociosas após 10min
        .max_lifetime(std::time::Duration::from_secs(1800))   // Vida máxima de uma conexão: 30min
        .after_connect(|conn, _meta| Box::pin(async move {
            // Configurações por conexão (ex: timezone, search_path)
            sqlx::query("SET timezone = 'America/Sao_Paulo'")
                .execute(conn)
                .await?;
            Ok(())
        }))
        .connect(&database_url)
        .await?;

    Ok(pool)
}
