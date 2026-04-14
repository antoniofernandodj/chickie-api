use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;
use chickie_core::{models::Usuario, proto};

pub async fn criar_adicional(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::CreateAdicionalRequest>,
) -> Result<Protobuf<proto::Adicional>, AppError> {

    let adicional = state.catalogo_service.criar_adicional(
        p.nome,
        loja_uuid,
        p.descricao,
        Decimal::from_str_exact(&p.preco)
            .map_err(|e| AppError::BadRequest(format!("Preço inválido: {}", e)))?
    ).await?;

    Ok(Protobuf(adicional.to_proto()))
}
