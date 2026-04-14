use axum::extract::{Path, State};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::ports::to_proto::ToProto;
use chickie_core::proto;
use crate::handlers::{AppState, auth::AdminPermission, dto::AppError, protobuf::Protobuf};

pub async fn adicionar_entregador(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    AdminPermission(_): AdminPermission,
    Protobuf(p): Protobuf<proto::AdicionarEntregadorRequest>,
) -> Result<Protobuf<proto::Entregador>, AppError> {

    let entregador = state.loja_service.adicionar_entregador(
        loja_uuid,
        p.nome,
        p.username,
        p.email,
        p.senha,
        p.celular,
        if p.veiculo.is_empty() { None } else { Some(p.veiculo) },
        if p.placa.is_empty() { None } else { Some(p.placa) }
    ).await?;

    Ok(Protobuf(entregador.to_proto()))
}
