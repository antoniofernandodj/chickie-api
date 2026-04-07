use axum::{Router, middleware::from_fn_with_state, routing::{get, post, put}};
use std::sync::Arc;

use crate::api::{AppState, auth_middleware};
use crate::api::{
    criar_pedido,
    listar_pedidos,
    listar_por_loja,
    buscar_pedido,
    buscar_pedido_com_entrega,
    atualizar_status,
};

pub fn pedido_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/criar", post(criar_pedido))
        .route("/listar", get(listar_pedidos))
        .route("/por-loja/{loja_uuid}", get(listar_por_loja))
        .route("/{uuid}", get(buscar_pedido))
        .route("/{uuid}/com-entrega", get(buscar_pedido_com_entrega))
        .route("/{uuid}/status", put(atualizar_status))
        .layer(from_fn_with_state(s.clone(), auth_middleware))
}
