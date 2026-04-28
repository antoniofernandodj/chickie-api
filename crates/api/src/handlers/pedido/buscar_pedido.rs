use axum::{
    Json, extract::{Path, State}, response::{IntoResponse}
};
use uuid::Uuid;

use std::sync::Arc;
use crate::{handlers::dto::AppError};
use crate::handlers::AppState;


pub async fn buscar_pedido(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let pedido = state
        .pedido_service
        .buscar_por_uuid(uuid)
        .await
        .map_err(AppError::Internal)?;

    Ok(Json(pedido))
}
