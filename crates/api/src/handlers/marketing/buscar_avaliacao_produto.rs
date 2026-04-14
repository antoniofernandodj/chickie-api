use std::sync::Arc;
use axum::{
    extract::{Path, State, Extension},
};
use uuid::Uuid;
use chickie_core::{
    models::Usuario,
    ports::to_proto::ToProto,
    usecases::MarketingUsecase,
    proto,
};
use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};

pub async fn buscar_avaliacao_produto(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::AvaliacaoProduto>, AppError> {
    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        Uuid::nil(), // not needed for this operation
        usuario
    );
    let avaliacao = usecase.buscar_avaliacao_produto(uuid).await?;
    Ok(Protobuf(avaliacao.to_proto()))
}
