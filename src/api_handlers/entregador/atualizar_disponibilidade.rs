use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api_handlers::{AppState, dto::AppError},
    models::Usuario,
    usecases::AdminUsecase,
};

#[derive(Deserialize)]
pub struct AtualizarDisponibilidadeEntregadorRequest {
    pub disponivel: bool,
}

pub async fn atualizar_disponibilidade_entregador(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, entregador_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<AtualizarDisponibilidadeEntregadorRequest>,
) -> Result<impl IntoResponse, AppError> {
    let admin_usecase = AdminUsecase::new(
        state.ingrediente_service.clone(),
        state.horario_funcionamento_service.clone(),
        state.config_pedido_service.clone(),
        state.funcionario_service.clone(),
        state.entregador_service.clone(),
        state.marketing_service.clone(),
        state.endereco_loja_service.clone(),
        usuario,
        loja_uuid,
    );

    admin_usecase
        .definir_entregador_disponivel(entregador_uuid, p.disponivel)
        .await
        .map_err(|e| AppError::BadRequest(e))?;

    Ok(StatusCode::NO_CONTENT)
}
