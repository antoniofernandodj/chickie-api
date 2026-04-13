use axum::Router;
use axum::routing::{get, patch, put, post};

use crate::handlers::{
    listar_usuarios,
    marcar_usuario_remocao,
    desmarcar_usuario_remocao,
    alternar_usuario_ativo,
    toggle_usuario_bloqueado,
};

pub fn usuario_routes() -> Router<std::sync::Arc<crate::handlers::AppState>> {
    Router::new()
        .route("/", get(listar_usuarios))
        .route("/{usuario_uuid}/marcar-remocao", patch(marcar_usuario_remocao))
        .route("/{usuario_uuid}/desmarcar-remocao", patch(desmarcar_usuario_remocao))
        .route("/{usuario_uuid}/ativo", put(alternar_usuario_ativo))
        .route("/{usuario_uuid}/bloqueado", post(toggle_usuario_bloqueado))
}
