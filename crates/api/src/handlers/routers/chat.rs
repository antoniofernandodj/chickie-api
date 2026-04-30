use axum::{routing::{get, put}, Router};
use std::sync::Arc;
use crate::handlers::{AppState, ws_chat_handler, listar_historico_pedido, listar_historico_loja_usuario, marcar_lida};

pub fn chat_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/ws", get(ws_chat_handler))
        .route("/historico/pedido/{pedido_uuid}", get(listar_historico_pedido))
        .route("/historico/loja/{loja_uuid}/usuario/{usuario_uuid}", get(listar_historico_loja_usuario))
        .route("/mensagens/{mensagem_uuid}/lida", put(marcar_lida))
}
