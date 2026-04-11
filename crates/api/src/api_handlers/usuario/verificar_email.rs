use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;

use crate::api_handlers::dto::{DisponivelResponse, VerificarEmailRequest};
use crate::api_handlers::dto::AppError;
use crate::api_handlers::AppState;

pub async fn verificar_email(
    State(state): State<Arc<AppState>>,
    Json(body): Json<VerificarEmailRequest>,
) -> Result<impl IntoResponse, AppError> {
    let disponivel = state
        .usuario_service
        .verificar_email_disponivel(&body.email)
        .await
        .map_err(|e| AppError::Internal(e))?;

    Ok(Json(DisponivelResponse { disponivel }))
}
