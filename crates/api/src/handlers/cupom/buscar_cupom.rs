use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use uuid::Uuid;
use std::sync::Arc;

use crate::handlers::{AppState, dto::AppError};
use chickie_core::models::Usuario;

pub async fn buscar_cupom(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let cupom = state.marketing_service.buscar_cupom(uuid).await?;

    Ok(Json(cupom))
}
