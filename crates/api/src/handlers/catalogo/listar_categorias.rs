use axum::{Json, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use crate::handlers::{AppState, dto::AppError};

pub async fn listar_categorias(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<Json<Vec<chickie_core::models::CategoriaProdutos>>, AppError> {
    let categorias = state.catalogo_service.listar_categorias(loja_uuid).await?;
    Ok(Json(categorias))
}
