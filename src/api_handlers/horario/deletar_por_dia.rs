use axum::{Extension, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use uuid::Uuid;
use std::sync::Arc;

use crate::{api_handlers::{dto::AppError, AppState}, models::Usuario, usecases::AdminUsecase};

pub async fn deletar_horario_dia(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, dia_semana)): Path<(Uuid, i32)>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {
    let uc = AdminUsecase::new(
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
    uc.deletar_horario_dia(dia_semana).await?;
    Ok(StatusCode::NO_CONTENT)
}
