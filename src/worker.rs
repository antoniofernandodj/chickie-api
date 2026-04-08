mod models;
mod database;
mod utils;
mod repositories;
mod services;
mod usecases;

use std::sync::Arc;
use tracing::{info, error};
use tracing_subscriber::fmt;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    // Initialize logging
    fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🔧 [WORKER] Chickie worker starting...");
    info!("🔧 [WORKER] PID: {}", std::process::id());

    // let pool = Arc::new(
    //     database::criar_pool()
    //         .await
    //         .expect("Failed to create database pool")
    // );

    // // Apply migrations (same as API)
    // database::aplicar_migrations(&pool)
    //     .await
    //     .expect("Failed to apply migrations");

    info!("🔧 [WORKER] Database initialized, starting worker loop...");

    // Main worker loop
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    loop {
        interval.tick().await;

        info!("🔧 [WORKER] Running scheduled tasks...");

        info!("✅ [WORKER] Scheduled tasks completed");

        // TODO: Implement background tasks here
        // Examples:
        // - Process pending notifications
        // - Clean up expired sessions
        // - Update order statuses
        // - Send scheduled emails
        // - Aggregate analytics

        // if let Err(e) = run_worker_tasks(&pool).await {
        //     error!("❌ [WORKER] Task execution failed: {}", e);
        // } else {
        //     info!("✅ [WORKER] Scheduled tasks completed");
        // }
    }
}

/// Execute all background worker tasks
async fn run_worker_tasks(pool: &Arc<sqlx::PgPool>) -> Result<(), anyhow::Error> {
    // TODO: Implement actual worker tasks
    // Example structure:
    //
    // // 1. Process pending notifications
    // process_pending_notifications(pool).await?;
    //
    // // 2. Clean up expired data
    // cleanup_expired_data(pool).await?;
    //
    // // 3. Update order statistics
    // update_order_statistics(pool).await?;

    info!("🔧 [WORKER] No tasks configured yet - worker loop idle");
    Ok(())
}
