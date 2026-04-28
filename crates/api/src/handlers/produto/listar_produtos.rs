use crate::handlers::dto::AppError;
use crate::handlers::AppState;
use axum::{Json, extract::{Path, State}};
use uuid::Uuid;
use std::sync::Arc;
use chickie_core::models;

pub async fn listar_produtos(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<Json<Vec<models::Produto>>, AppError> {
    let produtos = state.catalogo_service
        .listar_produtos_de_loja(loja_uuid)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(produtos))
}
