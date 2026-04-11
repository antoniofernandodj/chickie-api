use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::models::Usuario;
use crate::api_handlers::{AppState, dto::AppError};

#[derive(Deserialize)]
pub struct UpdateCategoriaRequest {
    pub nome: String,
    pub descricao: Option<String>,
    pub ordem: Option<i32>,
    #[serde(default)]
    pub pizza_mode: bool,
}

pub async fn atualizar_categoria(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, uuid)): Path<(Uuid, Uuid)>,
    Extension(_): Extension<Usuario>,
    Json(p): Json<UpdateCategoriaRequest>,
) -> Result<impl IntoResponse, AppError> {

    let categoria = state.catalogo_service.atualizar_categoria(
        uuid,
        loja_uuid,
        p.nome,
        p.descricao,
        p.ordem,
        p.pizza_mode,
    ).await?;

    Ok(Json(categoria))
}
