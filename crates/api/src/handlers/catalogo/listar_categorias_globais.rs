use axum::{Json, extract::State};
use std::sync::Arc;

use crate::handlers::{AppState, dto::AppError};

pub async fn listar_categorias_globais(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<chickie_core::models::CategoriaProdutos>>, AppError> {
    let categorias = state.catalogo_service.listar_categorias_globais().await?;
    Ok(Json(categorias))
}
