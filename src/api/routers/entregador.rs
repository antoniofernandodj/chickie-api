use axum::Router;
use axum::routing::{get, put};

use crate::api::{
    listar_entregadores,
    atualizar_entregador,
    entregador_trocar_email_senha,
};

pub fn entregador_routes() -> Router<std::sync::Arc<crate::api::AppState>> {
    Router::new()
        .route("/{loja_uuid}", get(listar_entregadores))
        .route("/{loja_uuid}/{uuid}", put(atualizar_entregador))
        .route("/{loja_uuid}/usuarios/{usuario_uuid}/credenciais", put(entregador_trocar_email_senha))
}
