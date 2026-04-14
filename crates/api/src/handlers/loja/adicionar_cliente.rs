use axum::extract::{Path, State};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::ports::to_proto::ToProto;
use chickie_core::proto;
use crate::handlers::{AppState, auth::AdminPermission, dto::AppError, protobuf::Protobuf};

pub async fn adicionar_cliente(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    AdminPermission(_): AdminPermission,
    Protobuf(p): Protobuf<proto::AdicionarClienteRequest>,
) -> Result<Protobuf<proto::Cliente>, AppError> {

    let cliente = state.loja_service.adicionar_cliente(
        loja_uuid,
        p.nome,
        p.username,
        p.email,
        p.senha,
        p.celular,
    ).await?;

    Ok(Protobuf(cliente.to_proto()))
}
