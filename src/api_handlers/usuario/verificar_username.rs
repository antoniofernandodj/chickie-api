use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;

use crate::api_handlers::dto::{DisponivelResponse, VerificarUsernameRequest};
use crate::api_handlers::dto::AppError;
use crate::api_handlers::AppState;

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
