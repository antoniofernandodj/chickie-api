use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState};

pub async fn buscar_pedido_por_codigo(
    State(state): State<Arc<AppState>>,
    Path(codigo): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let pedido = state
        .pedido_service
        .buscar_por_codigo(&codigo)
        .await
        .map_err(AppError::Internal)?;

    Ok(Json(pedido))
}
