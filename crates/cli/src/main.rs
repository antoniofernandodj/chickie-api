use std::sync::Arc;

use clap::Parser;

mod app_state;
mod args;
mod cmds;
mod database;
mod helpers;

use app_state::AppState;
use cmds::Commands;
use anyhow::Result;

#[derive(Parser)]
#[command(
    name = "chickie",
    about = "CLI para gerenciar o Chickie — acesso completo à API via terminal",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    let cli = Cli::parse();

    let pool = Arc::new(
        database::criar_pool()
            .await
            .map_err(|e| {
                eprintln!("❌ Falha ao conectar ao banco: {}", e);
                anyhow::anyhow!("Database connection failed")
            })?
    );

    let state = AppState::new(pool.clone());

    cmds::dispatch(cli.command, &state).await;

    Ok(())
}
