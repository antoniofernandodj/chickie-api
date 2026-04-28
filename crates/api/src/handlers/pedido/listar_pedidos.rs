use axum::{Json, extract::State, response::IntoResponse};
use std::sync::Arc;
use crate::handlers::{dto::AppError, AppState};

pub async fn listar_pedidos(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let pedidos = state
        .pedido_service
        .listar_todos()
        .await
        .map_err(AppError::Internal)?;

    Ok(Json(pedidos))
}
