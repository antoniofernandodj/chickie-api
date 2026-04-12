use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json
};
use serde::Deserialize;
use std::sync::Arc;

use chickie_core::usecases::LojaUsecase;
use crate::handlers::dto::AppError;
use crate::handlers::AppState;

#[derive(Deserialize)]
pub struct PesquisaQuery {
    pub termo: String,
}

pub async fn pesquisar_lojas(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PesquisaQuery>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = LojaUsecase::new(state.loja_service.clone());
    
    let lojas = usecase
        .pesquisar_lojas(&query.termo)
        .await
        .map_err(|e| AppError::Internal(e))?;

    Ok(Json(lojas))
}
