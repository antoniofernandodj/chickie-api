use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api_handlers::dto::AppError,
    api_handlers::AppState,
    usecases::LojaUsecase
};

pub async fn buscar_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let usecase = LojaUsecase::new(state.loja_service.clone());
    
    let loja = usecase
        .buscar_loja(loja_uuid)
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
