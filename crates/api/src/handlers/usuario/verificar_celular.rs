use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;

use crate::handlers::dto::{DisponivelResponse, AppError};
use crate::handlers::AppState;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct VerificarCelularRequest {
    pub celular: String,
}

pub async fn verificar_celular(
    State(state): State<Arc<AppState>>,
    Json(body): Json<VerificarCelularRequest>,
) -> Result<impl IntoResponse, AppError> {
    let disponivel = state
        .usuario_service
        .verificar_celular_disponivel(&body.celular)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(DisponivelResponse { disponivel }))
}
