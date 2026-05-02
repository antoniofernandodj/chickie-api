use axum::{
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use crate::handlers::{whatsapp, AppState};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/whatsapp", post(whatsapp::webhook::handler))
        .route("/whatsapp/delivery-status", post(whatsapp::delivery_status::handler))
}
