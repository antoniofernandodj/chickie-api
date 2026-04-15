use std::sync::Arc;

use axum::{
    extract::{Path, State, Extension}
};
use rust_decimal::Decimal;
use uuid::Uuid;
use chickie_core::{
    models::Usuario,
    usecases::MarketingUsecase,
    proto
};
use crate::handlers::{
    dto::AppError,
    AppState,
    protobuf::Protobuf
};
use chickie_core::{ports::to_proto::ToProto, models};
use rust_decimal::prelude::FromPrimitive;

pub async fn avaliar_produto(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(payload): Protobuf<proto::AvaliarProdutoRequest>,
) -> Result<Protobuf<proto::AvaliacaoProduto>, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    let produto_uuid = Uuid::parse_str(&payload.produto_uuid)
        .map_err(|e| AppError::BadRequest(format!("Invalid produto_uuid: {}", e)))?;
    let nota = payload.nota.parse::<f64>().unwrap_or_default();
    let descricao: Option<String> = if payload.descricao.is_empty() { None } else { Some(payload.descricao.clone()) };
    let comentario: Option<String> = if payload.comentario.is_empty() { None } else { Some(payload.comentario.clone()) };

    let avaliacao: models::AvaliacaoDeProduto = usecase.avaliar_produto(
        produto_uuid,
        Decimal::from_f64(nota).expect("Invalid f64 to Decimal conversion"),
        descricao.unwrap_or_default(),
        comentario
    ).await?;

    Ok(Protobuf(avaliacao.to_proto()))
}
