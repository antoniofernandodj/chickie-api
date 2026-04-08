use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;

use crate::api::dto::{DisponivelResponse, VerificarUsernameRequest};
use crate::api::dto::AppError;
use crate::api::AppState;

pub async fn verificar_username(
    State(state): State<Arc<AppState>>,
    Json(body): Json<VerificarUsernameRequest>,
) -> Result<impl IntoResponse, AppError> {
    let disponivel = state
        .usuario_service
        .verificar_username_disponivel(&body.username)
        .await
        .map_err(|e| AppError::Internal(e))?;

    Ok(Json(DisponivelResponse { disponivel }))
}
