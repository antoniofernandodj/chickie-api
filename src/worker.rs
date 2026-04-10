pub mod worker_lib;

mod models;
mod database;
mod utils;
mod repositories;
mod services;
mod usecases;
mod worker_handlers;

use tracing::info;
use tracing_subscriber::FmtSubscriber;
use worker_handlers::register::register_handlers;
use anyhow::Result;
use worker_lib::{Worker, WorkerConfig};


#[tokio::main]
async fn main() -> Result<()> {

    FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🐣 Chickie Worker iniciando...");

    let cfg: WorkerConfig = WorkerConfig::from_env();
    let mut worker: Worker = Worker::new(cfg);

    // ✨ Registra handlers — simples como Flask routes
    for item in register_handlers() {
        let handler = item.handler.clone();
        worker.queue(
            &item.queue,
            &item.routing_key,
            move |body| handler(body),
        );
    }

    worker.run().await
}
