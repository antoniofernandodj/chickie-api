use axum::{Json, extract::{Path, State}, response::IntoResponse};
use uuid::Uuid;
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState};

pub async fn buscar_config_pedido(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let config = state.config_pedido_service.buscar(loja_uuid).await?;
    match config {
        Some(c) => Ok(Json(c)),
        None => Err(AppError::NotFound("Configuração de pedido não encontrada".to_string())),
    }
}
