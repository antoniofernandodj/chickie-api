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

pub async fn listar_avaliacoes_produto_por_produto(
    State(state): State<Arc<AppState>>,
    Path(produto_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {
    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        Uuid::nil(), // not needed for this operation
        usuario
    );
    let avaliacoes = usecase.listar_avaliacoes_produto_por_produto(produto_uuid).await?;
    Ok(Json(avaliacoes))
}
