use axum::{Json, extract::State, response::IntoResponse};
use std::sync::Arc;

use crate::{
    api_handlers::{dto::AppError, AppState},
};

pub async fn listar_lojas_admin(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {

    let lojas = state.loja_service.listar().await?;

    Ok(Json(lojas))
}
