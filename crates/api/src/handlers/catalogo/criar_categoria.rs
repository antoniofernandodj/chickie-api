use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::models::Usuario;
use crate::handlers::{dto::AppError, AppState};

#[derive(Deserialize)]
pub struct CreateCategoriaRequest {
    pub nome: String,
    pub descricao: Option<String>,
    #[serde(default)]
    pub pizza_mode: bool,
    #[serde(default)]
    pub drink_mode: bool,
}

pub async fn criar_categoria(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
    Json(p): Json<CreateCategoriaRequest>,
) -> Result<impl IntoResponse, AppError> {

    if p.pizza_mode && p.drink_mode {
        return Err(AppError::BadRequest("Uma categoria não pode ter pizza_mode e drink_mode ativos ao mesmo tempo".to_string()));
    }

    let categoria = state.catalogo_service.criar_categoria(
        p.nome,
        p.descricao,
        Some(loja_uuid),
        p.pizza_mode,
        p.drink_mode
    ).await?;

    Ok(Json(categoria))
}
