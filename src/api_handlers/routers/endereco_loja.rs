use axum::Router;
use axum::routing::{get, post, put, delete};

use crate::api_handlers::{
    listar_enderecos_loja,
    criar_endereco_loja,
    atualizar_endereco_loja,
    deletar_endereco_loja,
};

pub fn endereco_loja_routes() -> Router<std::sync::Arc<crate::api_handlers::AppState>> {
    Router::new()
        .route("/{loja_uuid}", get(listar_enderecos_loja))
        .route("/{loja_uuid}", post(criar_endereco_loja))
        .route("/{loja_uuid}/{endereco_uuid}", put(atualizar_endereco_loja))
        .route("/{loja_uuid}/{endereco_uuid}", delete(deletar_endereco_loja))
}
