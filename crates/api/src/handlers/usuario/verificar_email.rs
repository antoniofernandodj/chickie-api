use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;

use crate::handlers::dto::{DisponivelResponse, VerificarEmailRequest};
use crate::handlers::dto::AppError;
use crate::handlers::AppState;

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
