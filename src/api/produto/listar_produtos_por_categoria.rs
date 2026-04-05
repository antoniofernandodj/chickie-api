use axum::{Extension, Json, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::{AppState, dto::AppError},
    models::Usuario,
};

pub async fn listar_produtos_por_categoria(
    State(state): State<Arc<AppState>>,
    Path(categoria_uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
) -> Result<Json<Vec<crate::models::Produto>>, AppError> {

    let produtos = state.catalogo_service.listar_produtos_por_categoria(categoria_uuid).await?;
    Ok(Json(produtos))
}
