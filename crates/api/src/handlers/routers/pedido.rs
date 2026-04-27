use axum::{Router, middleware::from_fn_with_state, routing::{get, put, delete, post}};
use std::sync::Arc;

use crate::handlers::{
    AppState,
    auth_middleware,
    optional_auth_middleware
};
use crate::handlers::{
    criar_pedido,
    listar_pedidos,
    // listar_por_loja,
    ws_listar_por_loja,
    ws_buscar_por_codigo,
    buscar_pedido,
    // buscar_pedido_por_codigo,
    buscar_pedido_com_entrega,
    atualizar_status,
    avancar_status,
    cancelar_pedido,
    listar_meus_pedidos,
    atribuir_entregador,
    remover_entregador,
    buscar_pedido_com_entregador,
};

pub fn pedido_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    // Rota sem auth obrigatória (usuário pode ser anônimo)
    let public_routes = Router::new()
        .route("/criar", post(criar_pedido))
        // .route("/codigo/{codigo}", get(buscar_pedido_por_codigo))
        .layer(from_fn_with_state(s.clone(), optional_auth_middleware));

    // Rotas que exigem autenticação via header Authorization
    let auth_routes = Router::new()
        .route("/listar", get(listar_pedidos))
        .route("/meus", get(listar_meus_pedidos))
        // .route("/por-loja/{loja_uuid}", get(listar_por_loja))
        .route("/{uuid}", get(buscar_pedido))
        .route("/{uuid}/com-entrega", get(buscar_pedido_com_entrega))
        .route("/{uuid}/status", put(atualizar_status))
        .route("/{uuid}/avancar", post(avancar_status))
        .route("/{uuid}/cancelar", post(cancelar_pedido))
        .route("/{uuid}/com-entregador", get(buscar_pedido_com_entregador))
        .route("/{pedido_uuid}/entregador/{loja_uuid}", put(atribuir_entregador))
        .route("/{pedido_uuid}/entregador/{loja_uuid}", delete(remover_entregador))
        .layer(from_fn_with_state(s.clone(), auth_middleware));

    // WebSocket: browser não suporta headers customizados no handshake,
    // então o token é recebido via query param e validado dentro do handler.
    let ws_routes = Router::new()
        .route("/por-loja/{loja_uuid}/ws", get(ws_listar_por_loja))
        .route("/codigo/{codigo}/ws", get(ws_buscar_por_codigo));

    Router::new()
        .merge(public_routes)
        .merge(auth_routes)
        .merge(ws_routes)
}
