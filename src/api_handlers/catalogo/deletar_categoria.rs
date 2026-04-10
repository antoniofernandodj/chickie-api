use axum::{Extension, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::{AppState, dto::AppError},
    models::Usuario,
};

pub async fn deletar_categoria(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, uuid)): Path<(Uuid, Uuid)>,
    Extension(_): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    state.catalogo_service.deletar_categoria(uuid, loja_uuid).await?;
    Ok(StatusCode::NO_CONTENT)
}
