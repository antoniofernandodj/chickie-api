use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::models::Usuario;
use chickie_core::proto;
use chickie_core::ports::to_proto::ToProto;
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn buscar_por_pedido(
    State(state): State<Arc<AppState>>,
    Path(pedido_uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
) -> Result<Protobuf<proto::Endereco>, AppError> {

    let endereco = state.endereco_entrega_service
        .buscar_por_pedido(pedido_uuid)
        .await?;

    let msg = "Endereço de entrega não encontrado para este pedido";
    match endereco {
        Some(e) => Ok(Protobuf(e.to_proto())),
        None => Err(AppError::NotFound(msg.to_string())),
    }
}
