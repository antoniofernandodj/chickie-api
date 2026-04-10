use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api_handlers::{dto::AppError, AppState},
    models::Usuario,
};

/// GET /api/pedidos/{pedido_uuid}/com-entregador
/// Busca um pedido com informações do entregador vinculado.
pub async fn buscar_pedido_com_entregador(
    State(state): State<Arc<AppState>>,
    Path(pedido_uuid): Path<Uuid>,
    Extension(_usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let pedido = state.pedido_service
        .buscar_pedido_com_entregador(pedido_uuid)
        .await?;

    Ok(Json(pedido))
}
