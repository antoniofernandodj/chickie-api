use std::sync::Arc;
use axum::{
    extract::{Path, State, Extension}, response::IntoResponse, http::StatusCode
};
use uuid::Uuid;
use chickie_core::{
    models::Usuario,
    usecases::MarketingUsecase
};
use crate::handlers::{AppState, dto::AppError};

pub async fn deletar_avaliacao_produto(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {
    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        Uuid::nil(), // not needed for this operation
        usuario
    );
    usecase.deletar_avaliacao_produto(uuid).await?;
    Ok(StatusCode::NO_CONTENT)
}
