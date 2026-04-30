use axum::{Json, extract::{Path, State}, response::IntoResponse};
use uuid::Uuid;
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState};

pub async fn verificar_loja_aberta(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let status = state.horario_funcionamento_service
        .verificar_aberta_agora(loja_uuid)
        .await?;
    Ok(Json(status))
}
