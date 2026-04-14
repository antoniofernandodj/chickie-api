use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use chickie_core::ports::to_proto::ToProto;
use chickie_core::{models::Usuario, proto};
use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};

pub async fn atualizar_adicional(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, adicional_uuid)): Path<(Uuid, Uuid)>,
    Extension(_): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::UpdateAdicionalRequest>,
) -> Result<Protobuf<proto::Adicional>, AppError> {

    let adicional = state.catalogo_service.atualizar_adicional(
        adicional_uuid,
        loja_uuid,
        p.nome,
        p.descricao,
        Decimal::from_str_exact(&p.preco)
            .map_err(|e| AppError::BadRequest(format!("Preço inválido: {}", e)))?,
    ).await?;

    Ok(Protobuf(adicional.to_proto()))
}
