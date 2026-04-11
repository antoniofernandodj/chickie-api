use axum::Router;
use axum::routing::{get, put};

use crate::api_handlers::{
    buscar_config_pedido,
    salvar_config_pedido,
};

pub fn config_pedido_routes() -> Router<std::sync::Arc<crate::api_handlers::AppState>> {
    Router::new()
        .route("/{loja_uuid}", get(buscar_config_pedido))
        .route("/{loja_uuid}", put(salvar_config_pedido))
}
