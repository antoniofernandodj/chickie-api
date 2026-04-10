use axum::Router;
use axum::routing::{get, post, put, delete};

use crate::api_handlers::{
    criar_endereco,
    listar_enderecos,
    buscar_endereco,
    atualizar_endereco,
    deletar_endereco,
};

pub fn endereco_usuario_routes() -> Router<std::sync::Arc<crate::api_handlers::AppState>> {
    Router::new()
        .route("/", post(criar_endereco))
        .route("/", get(listar_enderecos))
        .route("/{uuid}", get(buscar_endereco))
        .route("/{uuid}", put(atualizar_endereco))
        .route("/{uuid}", delete(deletar_endereco))
}
