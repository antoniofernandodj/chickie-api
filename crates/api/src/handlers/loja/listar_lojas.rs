use axum::{
    extract::{State},
    response::IntoResponse,
    Json
};


use std::sync::Arc;
use crate::handlers::dto::AppError;
use chickie_core::repositories::Repository;
use crate::handlers::AppState;


pub async fn listar_lojas(
    State(state): State<Arc<AppState>>
) -> Result<impl IntoResponse, AppError> {

    let lojas = state
        .loja_repo
        .listar_todos()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?; 

    Ok(Json(lojas))
}