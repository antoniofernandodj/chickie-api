mod models;
mod database;
mod utils;
mod repositories;
mod services;
mod usecases;
mod api;

use axum::{
    Json,
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::get
};

use std::{env, net::SocketAddr, sync::Arc};
use tower_http::cors::CorsLayer;
use tracing::{info, debug};
use tracing_subscriber::fmt;
use crate::api::AppState;
use serde_json::json;


#[tokio::main]
async fn main() {

    fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 [MAIN] Chickie starting...");
    info!("🚀 [MAIN] PID: {}", std::process::id());

    let port = env::var("APP_PORT")
        .unwrap_or_else(|_| String::from("3000"));

    info!("[MAIN] Starting on port {}...", port);

    let pool = Arc::new(
        database::criar_pool()
        .await
        .expect("Falha ao criar pool")
    );

    database::aplicar_migrations(&pool)
        .await
        .expect("Falha ao aplicar migrações");

    let s: Arc<AppState> = AppState::new(pool);

    let api_routes = api::api_routes(&s);
    let swagger_routes = api::swagger_router(&s);

    let app = Router::new()
        .route("/", get(handler_ok))
        .nest("/api", api_routes)
        .merge(swagger_routes)
        .fallback(handler_404)
        .layer(CorsLayer::permissive())
        .with_state(s);

    let port_num: u16 = port.parse()
        .expect("APP_PORT deve ser um número válido entre 1 e 65535");

    let addr = SocketAddr::from(([0, 0, 0, 0], port_num));

    let listener =
        tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    debug!("🚀 Servidor rodando em http://0.0.0.0:{}", port_num);
    axum::serve(listener, app).await.unwrap();

}


pub async fn handler_ok() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "message": "🚀 Servidor compilado com sucesso!"
        })),
    )
}


pub async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": "Rota não encontrada",
            "message": "A URL solicitada não existe neste servidor."
        })),
    )
}
