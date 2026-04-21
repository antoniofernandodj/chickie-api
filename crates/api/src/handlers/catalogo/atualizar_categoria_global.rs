use axum::{Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::handlers::{AppState, dto::AppError, OwnerPermission};

#[derive(Deserialize)]
pub struct UpdateCategoriaGlobalRequest {
    pub nome: String,
    pub descricao: Option<String>,
    #[serde(default)]
    pub pizza_mode: bool,
    #[serde(default)]
    pub drink_mode: bool,
}

pub async fn atualizar_categoria_global(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    _owner: OwnerPermission,
    Json(p): Json<UpdateCategoriaGlobalRequest>,
) -> Result<impl IntoResponse, AppError> {

    if p.pizza_mode && p.drink_mode {
        return Err(AppError::BadRequest("Uma categoria não pode ter pizza_mode e drink_mode ativos ao mesmo tempo".to_string()));
    }

    let categoria = state.catalogo_service.atualizar_categoria_global(
        uuid,
        p.nome,
        p.descricao,
        p.pizza_mode,
        p.drink_mode,
    ).await?;

    Ok(Json(categoria))
}
