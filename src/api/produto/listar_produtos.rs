use axum::{
    extract::{State},
    Json
};

use std::sync::Arc;
use crate::{api::dto::AppError, models, repositories::Repository};
use crate::api::AppState;


pub async fn listar_produtos(
    State(state): State<Arc<AppState>>
) -> Result<Json<Vec<models::Produto>>, AppError> {

    let produtos = state
        .produto_repo
        .listar_todos()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(produtos))
}