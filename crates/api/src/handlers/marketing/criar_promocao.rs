use axum::{
    Extension, extract::{Path, State}
};
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::Usuario,
    usecases::MarketingUsecase,
    proto
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;

pub async fn criar_promocao(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::CriarPromocaoRequest>,
) -> Result<Protobuf<proto::Promocao>, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    let produto_uuid = if p.produto_uuid.is_empty() { None } else { Some(Uuid::parse_str(&p.produto_uuid).unwrap_or_default()) };
    let categoria_uuid = if p.categoria_uuid.is_empty() { None } else { Some(Uuid::parse_str(&p.categoria_uuid).unwrap_or_default()) };
    let valor_desconto = if p.valor_desconto.is_empty() { None } else { Some(p.valor_desconto.parse().unwrap_or_default()) };
    let valor_minimo = if p.valor_minimo.is_empty() { None } else { Some(p.valor_minimo.parse().unwrap_or_default()) };
    let dias_semana: Option<Vec<u8>> = if p.dias_semana_validos.is_empty() { None } else { Some(p.dias_semana_validos.iter().map(|&x| x as u8).collect()) };

    let promocao = usecase.criar_promocao(
        p.nome,
        p.descricao,
        p.tipo_desconto,
        valor_desconto,
        valor_minimo,
        p.data_inicio,
        p.data_fim,
        dias_semana,
        p.tipo_escopo,
        produto_uuid,
        categoria_uuid,
        p.prioridade
    ).await?;

    Ok(Protobuf(promocao.to_proto()))
}
