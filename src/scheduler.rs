mod models;
mod database;
mod utils;
mod repositories;
mod services;
mod usecases;

use std::sync::Arc;
use tracing::{info, error, warn};
use tracing_subscriber::fmt;
use tokio::time::{Duration, interval};
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    // Initialize logging
    fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("⏰ [SCHEDULER] Chickie scheduler starting...");
    info!("⏰ [SCHEDULER] PID: {}", std::process::id());

    let pool = Arc::new(
        database::criar_pool()
            .await
            .expect("Failed to create database pool")
    );

    // Apply migrations (same as API)
    database::aplicar_migrations(&pool)
        .await
        .expect("Failed to apply migrations");

    info!("✅ [SCHEDULER] Database initialized, starting scheduled tasks...");

    // Run different tasks at different intervals
    let pool_clone = pool.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60)); // Every 1 minute
        loop {
            interval.tick().await;
            info!("⏰ [SCHEDULER] Running frequent tasks (1 min interval)...");
            if let Err(e) = run_frequent_tasks(&pool_clone).await {
                error!("❌ [SCHEDULER] Frequent tasks failed: {}", e);
            }
        }
    });

    let pool_clone = pool.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(300)); // Every 5 minutes
        loop {
            interval.tick().await;
            info!("⏰ [SCHEDULER] Running periodic tasks (5 min interval)...");
            if let Err(e) = run_periodic_tasks(&pool_clone).await {
                error!("❌ [SCHEDULER] Periodic tasks failed: {}", e);
            }
        }
    });

    let pool_clone = pool.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(3600)); // Every 1 hour
        loop {
            interval.tick().await;
            info!("⏰ [SCHEDULER] Running hourly tasks...");
            if let Err(e) = run_hourly_tasks(&pool_clone).await {
                error!("❌ [SCHEDULER] Hourly tasks failed: {}", e);
            }
        }
    });

    let pool_clone = pool.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(86400)); // Every 24 hours
        loop {
            interval.tick().await;
            info!("⏰ [SCHEDULER] Running daily tasks...");
            if let Err(e) = run_daily_tasks(&pool_clone).await {
                error!("❌ [SCHEDULER] Daily tasks failed: {}", e);
            }
        }
    });

    // Keep the main task alive
    loop {
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}

/// Frequent tasks (every 1 minute)
async fn run_frequent_tasks(_pool: &Arc<PgPool>) -> Result<(), anyhow::Error> {
    // TODO: Implement frequent tasks here
    // Examples:
    // - Check for order timeouts
    // - Update real-time statistics
    // - Process webhook events
    
    info!("📋 [SCHEDULER] Frequent tasks completed");
    Ok(())
}

/// Periodic tasks (every 5 minutes)
async fn run_periodic_tasks(_pool: &Arc<PgPool>) -> Result<(), anyhow::Error> {
    // TODO: Implement periodic tasks here
    // Examples:
    // - Sync order statuses
    // - Update delivery ETAs
    // - Check store availability
    
    info!("📋 [SCHEDULER] Periodic tasks completed");
    Ok(())
}

/// Hourly tasks (every 1 hour)
async fn run_hourly_tasks(_pool: &Arc<PgPool>) -> Result<(), anyhow::Error> {
    // TODO: Implement hourly tasks here
    // Examples:
    // - Aggregate analytics
    // - Send batch notifications
    // - Clean up temporary data
    // - Update store ratings
    
    info!("📋 [SCHEDULER] Hourly tasks completed");
    Ok(())
}

/// Daily tasks (every 24 hours)
async fn run_daily_tasks(pool: &Arc<PgPool>) -> Result<(), anyhow::Error> {
    // TODO: Implement daily tasks here
    // Examples:
    // - Soft-delete expired accounts (30 days after a_remover)
    // - Generate daily reports
    // - Archive old data
    // - Cleanup expired sessions
    // - Send daily summaries
    
    info!("🗑️ [SCHEDULER] Processing soft-deletes...");
    process_soft_deletes(pool).await?;
    
    info!("📊 [SCHEDULER] Generating daily reports...");
    generate_daily_reports(pool).await?;
    
    info!("📋 [SCHEDULER] Daily tasks completed");
    Ok(())
}

/// Process soft-deletes for users and stores
async fn process_soft_deletes(_pool: &Arc<PgPool>) -> Result<(), anyhow::Error> {
    // TODO: Implement soft-delete processing
    // According to QWEN.md:
    // - Users marked with a_remover = now() + 1 month
    // - After 30 days, mark excluida = true
    
    info!("🗑️ [SCHEDULER] Checking for expired soft-deletes...");
    
    // Example implementation structure:
    // 
    // // Soft-delete users (30 days after a_remover)
    // let result = sqlx::query!(
    //     r#"
    //     UPDATE usuarios 
    //     SET excluida = true, atualizado_em = NOW()
    //     WHERE a_remover IS NOT NULL 
    //     AND a_remover <= NOW() - INTERVAL '30 days'
    //     AND excluida = false
    //     "#
    // )
    // .execute(pool.as_ref())
    // .await?;
    // 
    // info!("🗑️ [SCHEDULER] Soft-deleted {} users", result.rows_affected());
    //
    // // Soft-delete stores (same mechanism)
    // let result = sqlx::query!(
    //     r#"
    //     UPDATE lojas 
    //     SET excluida = true, atualizado_em = NOW()
    //     WHERE a_remover IS NOT NULL 
    //     AND a_remover <= NOW() - INTERVAL '30 days'
    //     AND excluida = false
    //     "#
    // )
    // .execute(pool.as_ref())
    // .await?;
    // 
    // info!("🗑️ [SCHEDULER] Soft-deleted {} stores", result.rows_affected());
    
    warn!("⚠️ [SCHEDULER] Soft-delete processing not yet implemented");
    Ok(())
}

/// Generate daily reports
async fn generate_daily_reports(_pool: &Arc<PgPool>) -> Result<(), anyhow::Error> {
    // TODO: Implement daily report generation
    // Examples:
    // - Order statistics
    // - Revenue reports
    // - Store performance metrics
    // - User activity reports
    
    warn!("⚠️ [SCHEDULER] Daily report generation not yet implemented");
    Ok(())
}
