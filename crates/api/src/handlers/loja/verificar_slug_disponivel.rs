use axum::{
    extract::{Path, State},
};
use std::sync::Arc;

use chickie_core::{usecases::LojaUsecase, proto};
use crate::handlers::dto::AppError;
use crate::handlers::{AppState, protobuf::Protobuf};

pub async fn verificar_slug_disponivel(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Protobuf<proto::SlugDisponivelResponse>, AppError> {
    let usecase = LojaUsecase::new(state.loja_service.clone());

    let disponivel = usecase
        .verificar_slug_disponivel(&slug)
        .await
        .map_err(|e| AppError::Internal(e))?;

    Ok(Protobuf(proto::SlugDisponivelResponse {
        disponivel,
        slug,
    }))
}
