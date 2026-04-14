use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use crate::handlers::{AppState, dto::AppError, dto::DisponivelResponse};

pub async fn verificar_celular(
    State(state): State<Arc<AppState>>,
    Path(celular): Path<String>,
) -> Result<Json<DisponivelResponse>, AppError> {
    let disponivel = state.usuario_service
        .verificar_celular_disponivel(&celular)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(DisponivelResponse { disponivel }))
}
