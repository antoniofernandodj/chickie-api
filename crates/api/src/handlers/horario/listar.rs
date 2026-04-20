use axum::{Json, extract::{Path, State}, response::IntoResponse};
use uuid::Uuid;
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState};

pub async fn listar_horarios(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let horarios = state.horario_funcionamento_service
        .listar_por_loja(loja_uuid)
        .await?;
    Ok(Json(horarios))
}
