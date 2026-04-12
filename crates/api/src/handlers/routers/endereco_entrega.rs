use axum::Router;
use axum::routing::{get, post};

use crate::handlers::{
    criar_para_pedido,
    buscar_por_pedido,
    listar_enderecos_por_loja,
};

pub fn endereco_entrega_routes() -> Router<std::sync::Arc<crate::handlers::AppState>> {
    Router::new()
        .route("/{pedido_uuid}/{loja_uuid}", post(criar_para_pedido))
        .route("/{pedido_uuid}", get(buscar_por_pedido))
        .route("/{loja_uuid}/loja", get(listar_enderecos_por_loja))
}
