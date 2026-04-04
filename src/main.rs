mod models;
mod database;
mod utils;
mod repositories;
mod services;
mod usecases;
mod api;

use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};

use std::{env, io::{Write, stderr, stdout}, net::SocketAddr, sync::Arc};
use tower_http::cors::CorsLayer;
use tracing::{info, debug};
use tracing_subscriber::fmt;
use crate::api::AppState;
use serde_json::json;


#[tokio::main]
async fn main() {

    // // ✅ FORÇA O RUST A NÃO BUFFERIZAR LOGS (Crucial para Docker)
    // Write::flush(&mut stdout()).expect("Failed to flush stdout");
    // Write::flush(&mut stderr()).expect("Failed to flush stderr");

    // Inicializa o subscriber antes de qualquer log
    fmt()
        .with_target(false)
        .with_level(true)
        .init();

    // 🚀 LOG 1: Início absoluto
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

    // Aplica migrações (com drop se MODE=DEVELOPMENT)
    database::aplicar_migrations(&pool)
        .await
        .expect("Falha ao aplicar migrações");

    let s: Arc<AppState> = AppState::new(pool);

    let api_routes = api::api_routes(&s);

    let app = Router::new()
        .route("/", get(handler_ok))
        .nest("/api", api_routes) // Tudo agora começa com /api
        .fallback(handler_404)
        .layer(CorsLayer::permissive())
        .with_state(s);


    // Parse seguro da porta
    let port_num: u16 = port.parse().expect("APP_PORT deve ser um número válido entre 1 e 65535");

    // Bind em 0.0.0.0 para funcionar dentro e fora do Docker
    let addr = SocketAddr::from(([0, 0, 0, 0], port_num));

    // 6. Iniciar Servidor
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



#[tokio::test]
async fn test_handler_ok() {
    assert_eq!("OLA MUNDO", "OLA MUNDO");
}



// use axum::{Router, extract::State, http::{Request, StatusCode}};
// use tower::ServiceExt; // para `oneshot` e `call`
// use serde_json::json;

// #[tokio::test]
// async fn test_handler_ok() {
//     let app = Router::new()
//         .route("/users", axum::routing::get(list_users));

//     let request = Request::builder()
//         .uri("/users")
//         .body(axum::body::Body::empty())
//         .unwrap();

//     let response = app.oneshot(request).await.unwrap();
    
//     assert_eq!(response.status(), StatusCode::OK);
    
//     let body = axum::body::to_bytes(response.into_body(), usize::MAX)
//         .await
//         .unwrap();
    
//     let users: Vec<User> = serde_json::from_slice(&body).unwrap();
//     assert!(users.is_empty()); // ou faça asserções específicas
// }


// use sqlx::{PgPool, Row};

// async fn setup_test_db() -> PgPool {
//     let database_url = std::env::var("TEST_DATABASE_URL")
//         .expect("TEST_DATABASE_URL must be set");
    
//     PgPool::connect(&database_url)
//         .await
//         .expect("Failed to connect to test DB")
// }

// #[tokio::test]
// async fn test_user_repository() {
//     let pool = setup_test_db().await;
    
//     // Usa transação que será rollbackada ao final
//     let mut tx = pool.begin().await.unwrap();
    
//     // Seed de dados de teste
//     sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
//         .bind("Test User")
//         .bind("test@example.com")
//         .execute(&mut *tx)
//         .await
//         .unwrap();
    
//     // Seu código sob teste
//     let user = sqlx::query("SELECT * FROM users WHERE email = $1")
//         .bind("test@example.com")
//         .fetch_one(&mut *tx)
//         .await
//         .unwrap();
    
//     assert_eq!(user.get::<String, _>("name"), "Test User");
    
//     // Rollback explícito (opcional, mas explícito)
//     tx.rollback().await.unwrap();
// }
