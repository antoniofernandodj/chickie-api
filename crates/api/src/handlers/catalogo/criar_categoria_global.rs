use axum::{Json, extract::State, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState, OwnerPermission};

#[derive(Deserialize)]
pub struct CreateCategoriaGlobalRequest {
    pub nome: String,
    pub descricao: Option<String>,
    #[serde(default)]
    pub pizza_mode: bool,
    #[serde(default)]
    pub drink_mode: bool,
}

pub async fn criar_categoria_global(
    State(state): State<Arc<AppState>>,
    _owner: OwnerPermission,
    Json(p): Json<CreateCategoriaGlobalRequest>,
) -> Result<impl IntoResponse, AppError> {

    if p.pizza_mode && p.drink_mode {
        return Err(AppError::BadRequest("Uma categoria não pode ter pizza_mode e drink_mode ativos ao mesmo tempo".to_string()));
    }

    let categoria = state.catalogo_service.criar_categoria(
        p.nome,
        p.descricao,
        None, // Global category
        p.pizza_mode,
        p.drink_mode
    ).await?;

    Ok(Json(categoria))
}
