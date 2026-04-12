use axum::{
    Json, extract::{Path, State}, response::IntoResponse
};
use uuid::Uuid;

use std::sync::Arc;
use crate::{handlers::dto::AppError};
use crate::handlers::AppState;



pub async fn listar_pedidos(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {

    let pedidos = state
        .pedido_repo
        .buscar_por_loja(loja_uuid)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(pedidos))
}