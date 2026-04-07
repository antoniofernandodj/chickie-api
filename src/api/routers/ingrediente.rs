use axum::Router;
use axum::routing::{get, post, put, delete};

use crate::api::{
    criar_ingrediente,
    listar_ingredientes,
    atualizar_ingrediente,
    deletar_ingrediente,
};

pub fn ingrediente_routes() -> Router<std::sync::Arc<crate::api::AppState>> {
    Router::new()
        .route("/{loja_uuid}", post(criar_ingrediente))
        .route("/{loja_uuid}", get(listar_ingredientes))
        .route("/{loja_uuid}/{uuid}", put(atualizar_ingrediente))
        .route("/{loja_uuid}/{uuid}", delete(deletar_ingrediente))
}
