use axum::{Extension, Json, extract::State, response::IntoResponse};
use std::sync::Arc;

use crate::handlers::{AppState, dto::AppError};
use chickie_core::models::Usuario;

pub async fn listar_todos_cupons(
    State(state): State<Arc<AppState>>,
    Extension(_): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let cupons = state.marketing_service.listar_todos_cupons().await?;

    Ok(Json(cupons))
}
