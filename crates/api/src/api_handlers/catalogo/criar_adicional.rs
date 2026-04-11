use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;

use chickie_core::models::Usuario;
use crate::api_handlers::{dto::AppError, AppState};

#[derive(Deserialize)]
pub struct CreateAdicionalRequest {
    pub nome: String,
    pub descricao: String,
    pub preco: Decimal,
}

pub async fn criar_adicional(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
    Json(p): Json<CreateAdicionalRequest>,
) -> Result<impl IntoResponse, AppError> {

    let adicional = state.catalogo_service.criar_adicional(
        p.nome,
        loja_uuid,
        p.descricao,
        p.preco
    ).await?;

    Ok(Json(adicional))
}
