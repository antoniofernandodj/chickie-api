use axum::{
    extract::{State},
    response::IntoResponse,
    Json
};


use std::sync::Arc;
use crate::{api::dto::AppError, repositories::Repository};
use crate::api::AppState;


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