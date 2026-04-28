use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::models::Usuario;
use crate::handlers::{AppState, dto::AppError};

#[derive(Deserialize)]
pub struct UpdateCategoriaRequest {
    pub nome: String,
    pub descricao: Option<String>,
    #[serde(default)]
    pub pizza_mode: bool,
    #[serde(default)]
    pub drink_mode: bool,
}

pub async fn atualizar_categoria(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, uuid)): Path<(Uuid, Uuid)>,
    Extension(_): Extension<Usuario>,
    Json(p): Json<UpdateCategoriaRequest>,
) -> Result<impl IntoResponse, AppError> {

    if p.pizza_mode && p.drink_mode {
        return Err(AppError::BadRequest("Uma categoria não pode ter pizza_mode e drink_mode ativos ao mesmo tempo".to_string()));
    }

    let categoria = state.catalogo_service.atualizar_categoria(
        uuid,
        loja_uuid,
        p.nome,
        p.descricao,
        p.pizza_mode,
        p.drink_mode,
    ).await?;

    Ok(Json(categoria))
}
