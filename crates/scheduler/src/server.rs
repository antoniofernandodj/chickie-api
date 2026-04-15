use anyhow::Result;
use chrono::Utc;
use std::env;
use axum::{response::Json, routing::get, Router};
use serde_json::json;
use std::net::SocketAddr;

use crate::log::log_info;

pub async fn start_health_server() -> Result<()> {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/", get(health_check));

    let port = env::var("SCHEDULER_PORT")
        .unwrap_or_else(|_| "8080".to_string());
    
    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Invalid address");

    log_info(&format!("🌐 Health check server listening on http://{}", addr));

    axum::serve(
        tokio::net::TcpListener::bind(&addr).await?,
        app
    )
    .await
    .map_err(|e| anyhow::anyhow!("Health server error: {}", e))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "scheduler",
        "timestamp": Utc::now().to_rfc3339()
    }))
}