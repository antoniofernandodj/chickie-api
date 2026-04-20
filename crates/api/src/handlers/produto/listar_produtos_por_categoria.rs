use crate::handlers::{AppState, dto::AppError};
use axum::{Json, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;
use chickie_core::models::Produto;

pub async fn listar_produtos_por_categoria(
    State(state): State<Arc<AppState>>,
    Path(categoria_uuid): Path<Uuid>,
) -> Result<Json<Vec<Produto>>, AppError> {
    let produtos = state.catalogo_service
        .listar_produtos_por_categoria(categoria_uuid)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(produtos))
}
