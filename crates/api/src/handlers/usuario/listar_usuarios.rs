use axum::{
    extract::{State},
    Json
};
use std::sync::Arc;
use crate::handlers::{AppState, dto::AppError};
use chickie_core::{models, repositories::Repository};


pub async fn listar_usuarios(
    State(state): State<Arc<AppState>>
) -> Result<Json<Vec<models::Usuario>>, AppError> {

    let usuarios = state
        .usuario_repo
        .listar_todos()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(usuarios))
}