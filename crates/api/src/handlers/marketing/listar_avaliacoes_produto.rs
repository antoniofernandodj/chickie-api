use std::sync::Arc;
use axum::{
    extract::{Path, State, Extension},
};
use uuid::Uuid;
use chickie_core::{
    models::Usuario,
    usecases::MarketingUsecase,
    proto,
};
use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};

pub async fn listar_avaliacoes_produto_por_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::ListarAvaliacoesProdutoResponse>, AppError> {
    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );
    let avaliacoes = usecase.listar_avaliacoes_produto_por_loja().await?;

    let proto_avaliacoes: Vec<_> = avaliacoes.into_iter()
        .map(|a| a.to_proto())
        .collect();

    Ok(Protobuf(proto::ListarAvaliacoesProdutoResponse {
        avaliacoes: proto_avaliacoes,
    }))
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
use crate::handlers::{AppState, dto::AppError};

pub async fn listar_avaliacoes_produto_por_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {
    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );
    let avaliacoes = usecase.listar_avaliacoes_produto_por_loja().await?;
    Ok(Json(avaliacoes))
}


*/