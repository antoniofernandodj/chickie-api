use axum::{
    Extension, extract::State
};
use std::sync::Arc;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};
use chickie_core::{models::Usuario, proto};
use chickie_core::ports::to_proto::ToProto;

pub async fn criar_cupom_generico(
    Extension(_): Extension<Usuario>,
    State(state): State<Arc<AppState>>,
    Protobuf(p): Protobuf<proto::CriarCupomGenericoRequest>,
) -> Result<Protobuf<proto::Cupom>, AppError> {

    let loja_uuid = Uuid::parse_str(&p.loja_uuid)
        .map_err(|e| AppError::BadRequest(format!("Invalid loja_uuid: {}", e)))?;
    let valor_desconto = if p.valor_desconto.is_empty() { None } else { Some(p.valor_desconto.parse::<Decimal>().unwrap_or_default()) };
    let valor_minimo = if p.valor_minimo.is_empty() { None } else { Some(p.valor_minimo.parse::<Decimal>().unwrap_or_default()) };
    let limite_uso = if p.limite_uso == 0 { None } else { Some(p.limite_uso) };

    let cupom = state.marketing_service.criar_cupom(
        loja_uuid,
        p.codigo,
        p.descricao,
        p.tipo_desconto,
        valor_desconto,
        valor_minimo,
        p.data_validade,
        limite_uso
    ).await?;

    Ok(Protobuf(cupom.to_proto()))
}
