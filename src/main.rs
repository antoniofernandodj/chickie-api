mod models;
mod database;
mod utils;
mod repositories;
mod services;
mod api;

use axum::{Router, Json, http::StatusCode, response::IntoResponse};

use std::sync::Arc;
use tower_http::cors::CorsLayer;
use crate::api::AppState;


use serde_json::json;


#[tokio::main]
async fn main() {
    let pool = Arc::new(
        database::criar_pool()
        .await
        .expect("Falha ao criar pool")
    );

    let s: Arc<AppState> = AppState::new(pool);

    let api_routes = api::api_routes(&s);

    let app = Router::new()
        .nest("/api", api_routes) // Tudo agora começa com /api
        .fallback(handler_404)
        .layer(CorsLayer::permissive())
        .with_state(s);

    // 6. Iniciar Servidor
    let listener =
        tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Servidor rodando em http://localhost:3000");
    axum::serve(listener, app).await.unwrap();

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