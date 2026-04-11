use axum::{Extension, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::models::Usuario;
use crate::api_handlers::{dto::AppError, AppState};

pub async fn deletar_endereco(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    state.endereco_usuario_service
        .deletar_endereco(uuid, usuario.uuid)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
