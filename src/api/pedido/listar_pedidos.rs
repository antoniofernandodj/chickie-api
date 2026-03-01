use axum::{
    Json, extract::State, response::{IntoResponse}
};

use std::sync::Arc;
use crate::{api::dto::AppError};
use crate::repositories::Repository;
use crate::api::AppState;



pub async fn listar_pedidos(
    State(state): State<Arc<AppState>>
) -> Result<impl IntoResponse, AppError> {

    let pedidos = state
        .pedido_repo
        .listar_todos()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(pedidos))
}