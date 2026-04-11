use axum::{Router, middleware::from_fn_with_state, routing::{get, post, put, delete}};
use std::sync::Arc;

use crate::api_handlers::{AppState, auth_middleware};
use crate::api_handlers::{
    criar_cupom,
    validar_cupom,
    listar_cupons,
    avaliar_loja,
    avaliar_produto,
    criar_promocao,
    listar_promocoes,
    atualizar_promocao,
    deletar_promocao,
};

pub fn marketing_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/{loja_uuid}/cupons", post(criar_cupom))
        .route("/cupons/{codigo}", get(validar_cupom))
            .layer(from_fn_with_state(s.clone(), auth_middleware))
        .route("/cupons", get(listar_cupons))
        .route("/{loja_uuid}/avaliar-loja", post(avaliar_loja))
        .route("/{loja_uuid}/avaliar-produto", post(avaliar_produto))
        .route("/{loja_uuid}/promocoes", post(criar_promocao))
        .route("/{loja_uuid}/promocoes", get(listar_promocoes))
        .route("/{loja_uuid}/promocoes/{uuid}", put(atualizar_promocao))
        .route("/{loja_uuid}/promocoes/{uuid}", delete(deletar_promocao))
}
