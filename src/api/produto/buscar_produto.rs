use axum::{Extension, Json, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::{AppState, dto::AppError},
    models::Usuario,
};

pub async fn buscar_produto_por_uuid(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
) -> Result<Json<crate::models::Produto>, AppError> {

    let produto = state.catalogo_service.buscar_produto_por_uuid(uuid).await?;
    Ok(Json(produto))
}
