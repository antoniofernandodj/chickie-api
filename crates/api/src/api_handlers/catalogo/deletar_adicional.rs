use axum::{Extension, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::models::Usuario;
use crate::api_handlers::{AppState, dto::AppError};

pub async fn deletar_adicional(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, adicional_uuid)): Path<(Uuid, Uuid)>,
    Extension(_): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    state.catalogo_service.deletar_adicional(adicional_uuid, loja_uuid).await?;
    Ok(StatusCode::NO_CONTENT)
}
