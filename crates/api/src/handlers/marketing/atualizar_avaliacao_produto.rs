use std::sync::Arc;
use axum::{
    extract::{Path, State, Extension}
};
use uuid::Uuid;
use chickie_core::{
    models::Usuario,
    usecases::MarketingUsecase,
    proto
};
use crate::handlers::{
    AppState, dto::AppError, protobuf::Protobuf
};
use chickie_core::ports::to_proto::ToProto;
use rust_decimal::{Decimal, prelude::FromPrimitive};

pub async fn atualizar_avaliacao_produto(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(payload): Protobuf<proto::AtualizarAvaliacaoProdutoRequest>,
) -> Result<Protobuf<proto::AvaliacaoProduto>, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        Uuid::nil(), // not needed for this operation
        usuario
    );

    let nota = payload.nota.parse::<f64>().unwrap_or_default();
    let descricao = if payload.descricao.is_empty() { None } else { Some(payload.descricao.clone()) };
    let comentario = if payload.comentario.is_empty() { None } else { Some(payload.comentario.clone()) };
    let avaliacao = usecase.atualizar_avaliacao_produto(
        uuid,
        Decimal::from_f64(nota).unwrap_or(Decimal::ZERO),
        descricao.unwrap_or_default(),
        comentario
    ).await?;

    Ok(Protobuf(avaliacao.to_proto()))
}
