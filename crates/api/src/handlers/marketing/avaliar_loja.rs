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

pub async fn avaliar_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(payload): Protobuf<proto::AvaliarLojaRequest>,
) -> Result<Protobuf<proto::AvaliacaoDeLoja>, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    let nota = payload.nota.parse::<f64>().unwrap_or_default();
    let comentario = if payload.comentario.is_empty() { None } else { Some(payload.comentario.clone()) };

    let avaliacao = usecase.avaliar_loja(
        nota,
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
    dto::{AppError, AvaliarLojaRequest},
    AppState
};

pub async fn avaliar_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Json(payload): Json<AvaliarLojaRequest>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    let avaliacao = usecase.avaliar_loja(
        payload.nota,
        payload.comentario
    ).await?;

    Ok(Json(avaliacao))
}
*/
