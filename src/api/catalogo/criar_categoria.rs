use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::{dto::AppError, AppState},
    models::Usuario
};

#[derive(Deserialize)]
pub struct CreateCategoriaRequest {
    pub nome: String,
    pub descricao: Option<String>,
    pub ordem: Option<i32>,
}

pub async fn criar_categoria(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
    Json(p): Json<CreateCategoriaRequest>,
) -> Result<impl IntoResponse, AppError> {

    let categoria = state.catalogo_service.criar_categoria(
        p.nome,
        p.descricao,
        loja_uuid,
        p.ordem
    ).await?;

    Ok(Json(categoria))
}
