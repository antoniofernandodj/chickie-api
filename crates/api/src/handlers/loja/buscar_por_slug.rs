use axum::{
    extract::{Path, State}
};
use std::sync::Arc;

use chickie_core::usecases::LojaUsecase;
use chickie_core::proto;
use chickie_core::ports::to_proto::ToProto;
use crate::handlers::dto::AppError;
use crate::handlers::AppState;
use crate::handlers::protobuf::Protobuf;

pub async fn buscar_loja_por_slug(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Protobuf<proto::Loja>, AppError> {
    let usecase = LojaUsecase::new(state.loja_service.clone());

    let loja = usecase
        .buscar_loja_por_slug(&slug)
        .await
        .map_err(|e| {
            if e.contains("não encontrada") {
                AppError::NotFound(e)
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok(Protobuf(loja.to_proto()))
}
