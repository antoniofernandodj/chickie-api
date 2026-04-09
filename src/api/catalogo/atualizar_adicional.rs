use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::{
    api::{AppState, dto::AppError},
    models::Usuario,
};

#[derive(Deserialize)]
pub struct UpdateAdicionalRequest {
    pub nome: String,
    pub descricao: String,
    pub preco: Decimal,
}

pub async fn atualizar_adicional(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, adicional_uuid)): Path<(Uuid, Uuid)>,
    Extension(_): Extension<Usuario>,
    Json(p): Json<UpdateAdicionalRequest>,
) -> Result<impl IntoResponse, AppError> {

    let adicional = state.catalogo_service.atualizar_adicional(
        adicional_uuid,
        loja_uuid,
        p.nome,
        p.descricao,
        p.preco,
    ).await?;

    Ok(Json(adicional))
}
