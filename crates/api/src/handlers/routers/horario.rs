use axum::Router;
use axum::routing::{get, post, put, delete};

use crate::handlers::{
    listar_horarios,
    criar_ou_atualizar_horario,
    definir_ativo,
    deletar_horario_dia,
};

pub fn horario_routes() -> Router<std::sync::Arc<crate::handlers::AppState>> {
    Router::new()
        .route("/{loja_uuid}", get(listar_horarios))
        .route("/{loja_uuid}", post(criar_ou_atualizar_horario))
        .route("/{loja_uuid}/dia/{dia_semana}/ativo", put(definir_ativo))
        .route("/{loja_uuid}/dia/{dia_semana}", delete(deletar_horario_dia))
}
