use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::models::Usuario;
use crate::handlers::{dto::AppError, AppState};

pub async fn buscar_endereco(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let endereco = state.endereco_usuario_service
        .buscar_endereco(uuid, usuario.uuid)
        .await?;

    let msg = "Endereço não encontrado";
    match endereco {
        Some(e) => Ok(Json(e)),
        None => Err(AppError::NotFound(msg.to_string())),
    }
}
