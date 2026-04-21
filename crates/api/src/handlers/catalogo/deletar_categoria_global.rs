use axum::{extract::{Path, State}, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use uuid::Uuid;

use crate::handlers::{AppState, dto::AppError, OwnerPermission};

pub async fn deletar_categoria_global(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    _owner: OwnerPermission,
) -> Result<impl IntoResponse, AppError> {

    state.catalogo_service.deletar_categoria_global(uuid).await?;
    Ok(StatusCode::NO_CONTENT)
}
