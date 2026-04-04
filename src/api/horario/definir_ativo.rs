use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

use crate::{api::{dto::AppError, AppState}, models::Usuario, usecases::AdminUsecase};

#[derive(Deserialize)]
pub struct DefinirAtivoRequest {
    pub ativo: bool,
}

pub async fn definir_ativo(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, dia_semana)): Path<(Uuid, i32)>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<DefinirAtivoRequest>,
) -> Result<impl IntoResponse, AppError> {
    let uc = AdminUsecase::new(
        state.ingrediente_service.clone(),
        state.horario_funcionamento_service.clone(),
        state.config_pedido_service.clone(),
        state.funcionario_service.clone(),
        state.entregador_service.clone(),
        state.marketing_service.clone(),
        usuario,
        loja_uuid,
    );
    uc.definir_horario_ativo(dia_semana, p.ativo).await?;
    Ok(StatusCode::NO_CONTENT)
}
