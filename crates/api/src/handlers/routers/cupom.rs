use axum::{Router, middleware::from_fn_with_state, routing::{get, post, put, delete, patch}};
use std::sync::Arc;

use crate::handlers::{AppState, auth_middleware};
use crate::handlers::{
    criar_cupom,
    criar_cupom_generico,
    validar_cupom,
    listar_cupons,
    listar_todos_cupons,
    buscar_cupom,
    atualizar_cupom,
    deletar_cupom,
    atualizar_status_cupom,
};

pub fn cupom_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        // CRUD padronizado em /api/cupons/
        .route("/", post(criar_cupom_generico))
        .route("/", get(listar_todos_cupons))
        .route("/{uuid}", get(buscar_cupom))
        .route("/{uuid}", put(atualizar_cupom))
        .route("/{uuid}", delete(deletar_cupom))
        .route("/{uuid}/status", patch(atualizar_status_cupom))
        // Rotas legadas (manter compatibilidade)
        .route("/{loja_uuid}/cupons", post(criar_cupom))
        .route("/cupons/{codigo}", get(validar_cupom))
        .route("/cupons", get(listar_cupons))
        .layer(from_fn_with_state(s.clone(), auth_middleware))
}
