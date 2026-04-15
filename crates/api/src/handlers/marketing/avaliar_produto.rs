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
    dto::AppError,
    AppState,
    protobuf::Protobuf
};
use chickie_core::ports::to_proto::ToProto;

pub async fn avaliar_produto(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(payload): Protobuf<proto::AvaliarProdutoRequest>,
) -> Result<Protobuf<proto::AvaliacaoDeProduto>, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    let produto_uuid = Uuid::parse_str(&payload.produto_uuid)
        .map_err(|e| AppError::BadRequest(format!("Invalid produto_uuid: {}", e)))?;
    let nota = payload.nota.parse::<f64>().unwrap_or_default();
    let descricao = if payload.descricao.is_empty() { None } else { Some(payload.descricao.clone()) };
    let comentario = if payload.comentario.is_empty() { None } else { Some(payload.comentario.clone()) };

    let avaliacao = usecase.avaliar_produto(
        produto_uuid,
        nota,
        descricao,
        comentario
    ).await?;

    Ok(Protobuf(avaliacao.to_proto()))
}

/*
use std::sync::Arc;

use axum::{
    Json, extract::{Path, State, Extension}, response::IntoResponse
};
use uuid::Uuid;
use chickie_core::{
    models::Usuario,
    usecases::MarketingUsecase
};
use crate::handlers::{
    dto::{AppError, AvaliarProdutoRequest},
    AppState
};

pub async fn avaliar_produto(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Json(payload): Json<AvaliarProdutoRequest>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    let avaliacao = usecase.avaliar_produto(
        payload.produto_uuid,
        payload.nota,
        payload.descricao,
        payload.comentario
    ).await?;

    Ok(Json(avaliacao))
}
*/
