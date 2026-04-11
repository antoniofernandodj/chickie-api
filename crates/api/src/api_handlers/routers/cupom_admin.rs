use axum::Router;
use axum::routing::{put, delete};

use crate::api_handlers::{
    atualizar_cupom,
    deletar_cupom,
};

pub fn cupom_admin_routes() -> Router<std::sync::Arc<crate::api_handlers::AppState>> {
    Router::new()
        .route("/{loja_uuid}/{uuid}", put(atualizar_cupom))
        .route("/{loja_uuid}/{uuid}", delete(deletar_cupom))
}
