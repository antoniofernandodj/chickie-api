use axum::{Extension, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use uuid::Uuid;
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState};
use chickie_core::models::Usuario;

pub async fn deletar_cupom(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(_usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {
    state.marketing_service.deletar_cupom(uuid).await
        .map_err(|e| AppError::Internal(e))?;
    Ok(StatusCode::NO_CONTENT)
}
