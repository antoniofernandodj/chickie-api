use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json
};
use std::sync::Arc;

use chickie_core::usecases::LojaUsecase;
use crate::api_handlers::dto::AppError;
use crate::api_handlers::AppState;

pub async fn buscar_loja_por_slug(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let usecase = LojaUsecase::new(state.loja_service.clone());
    
    let loja = usecase
        .buscar_loja_por_slug(&slug)
        .await
        .map_err(|e| {
            if e.contains("não encontrada") {
                AppError::NotFound(e)
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok(Json(loja))
}
