use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::models::Usuario;
use crate::api_handlers::{dto::AppError, AppState};

pub async fn buscar_por_pedido(
    State(state): State<Arc<AppState>>,
    Path(pedido_uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let endereco = state.endereco_entrega_service
        .buscar_por_pedido(pedido_uuid)
        .await?;

    let msg = "Endereço de entrega não encontrado para este pedido";
    match endereco {
        Some(e) => Ok(Json(e)),
        None => Err(AppError::NotFound(msg.to_string())),
    }
}
