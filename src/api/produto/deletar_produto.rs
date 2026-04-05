use axum::{Extension, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::{AppState, dto::AppError},
    models::Usuario,
};

pub async fn deletar_produto(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    state.catalogo_service.deletar_produto(uuid).await?;
    Ok(StatusCode::NO_CONTENT)
}
