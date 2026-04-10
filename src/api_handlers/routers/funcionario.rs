use axum::Router;
use axum::routing::{get, put};

use crate::api::{
    listar_funcionarios,
    atualizar_funcionario,
    funcionario_trocar_email_senha,
};

pub fn funcionario_routes() -> Router<std::sync::Arc<crate::api::AppState>> {
    Router::new()
        .route("/{loja_uuid}", get(listar_funcionarios))
        .route("/{loja_uuid}/{uuid}", put(atualizar_funcionario))
        .route("/{loja_uuid}/usuarios/{usuario_uuid}/credenciais", put(funcionario_trocar_email_senha))
}
