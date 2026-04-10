use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use uuid::Uuid;
use std::sync::Arc;

use crate::{api::{dto::AppError, AppState}, models::Usuario, usecases::AdminUsecase};

pub async fn buscar_config_pedido(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
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
    let config = uc.buscar_config_pedido().await?;
    match config {
        Some(c) => Ok(Json(c)),
        None => Err(AppError::NotFound("Configuração de pedido não encontrada".to_string())),
    }
}
