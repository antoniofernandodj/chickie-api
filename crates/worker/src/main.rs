pub mod worker_lib;
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

    for i in register_handlers() {
        let h = i.handler.clone();
        let h = move |body| h(body);
        worker.queue(&i.queue, &i.routing_key, h);
    }

    worker.run().await
}
