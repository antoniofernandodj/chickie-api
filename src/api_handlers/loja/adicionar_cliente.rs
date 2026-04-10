use axum::{Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api_handlers::{AppState, auth::AdminPermission, dto::AppError},
};

#[derive(Deserialize)]
pub struct AdicionarClienteRequest {
    pub nome: String,
    pub username: String,
    pub email: String,
    pub senha: String,
    pub celular: String,
}

pub async fn adicionar_cliente(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    AdminPermission(_): AdminPermission,
    Json(p): Json<AdicionarClienteRequest>,
) -> Result<impl IntoResponse, AppError> {

    let cliente = state.loja_service.adicionar_cliente(
        loja_uuid,
        p.nome,
        p.username,
        p.email,
        p.senha,
        p.celular,
    ).await?;

    Ok(Json(cliente))
}
