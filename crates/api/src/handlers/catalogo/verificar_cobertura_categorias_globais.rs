use axum::{Json, extract::State};
use std::sync::Arc;

use crate::handlers::{AppState, dto::AppError};
use chickie_core::models::StatusCategoriaGlobal;

pub async fn verificar_cobertura_categorias_globais(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<StatusCategoriaGlobal>>, AppError> {
    let resultado = state.catalogo_service
        .verificar_cobertura_categorias_globais()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(resultado))
}
