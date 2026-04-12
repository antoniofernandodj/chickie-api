use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use std::sync::Arc;

use chickie_core::usecases::LojaUsecase;
use crate::handlers::dto::AppError;
use crate::handlers::AppState;

#[derive(Serialize)]
pub struct SlugDisponivelResponse {
    pub disponivel: bool,
    pub slug: String,
}

pub async fn verificar_slug_disponivel(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let usecase = LojaUsecase::new(state.loja_service.clone());

    let disponivel = usecase
        .verificar_slug_disponivel(&slug)
        .await
        .map_err(|e| AppError::Internal(e))?;

    Ok(Json(SlugDisponivelResponse {
        disponivel,
        slug,
    }))
}
