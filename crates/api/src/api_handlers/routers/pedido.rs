use axum::{Router, middleware::from_fn_with_state, routing::{get, put, delete, post}};
use std::sync::Arc;

use crate::api_handlers::{AppState, auth_middleware};
use crate::api_handlers::{
    criar_pedido,
    listar_pedidos,
    listar_por_loja,
    buscar_pedido,
    buscar_pedido_com_entrega,
    atualizar_status,
    listar_meus_pedidos,
    atribuir_entregador,
    remover_entregador,
    buscar_pedido_com_entregador,
};

pub fn pedido_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/criar", post(criar_pedido))
        .route("/listar", get(listar_pedidos))
        .route("/meus", get(listar_meus_pedidos))
        .route("/por-loja/{loja_uuid}", get(listar_por_loja))
        .route("/{uuid}", get(buscar_pedido))
        .route("/{uuid}/com-entrega", get(buscar_pedido_com_entrega))
        .route("/{uuid}/status", put(atualizar_status))
        .route("/{uuid}/com-entregador", get(buscar_pedido_com_entregador))
        .route("/{pedido_uuid}/entregador/{loja_uuid}", put(atribuir_entregador))
        .route("/{pedido_uuid}/entregador/{loja_uuid}", delete(remover_entregador))
        .layer(from_fn_with_state(s.clone(), auth_middleware))
}
