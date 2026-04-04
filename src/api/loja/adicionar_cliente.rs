use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::{AppState, auth::AdminPermission, dto::AppError},
    models::Usuario
};

#[derive(Deserialize)]
pub struct AdicionarClienteRequest {
    pub nome: String,
    pub username: String,
    pub email: String,
    pub senha: String,
    pub celular: String,
    // pub telefone: Option<String>,
}

pub async fn adicionar_cliente(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    AdminPermission(usuario): AdminPermission,
    Json(p): Json<AdicionarClienteRequest>,
) -> Result<impl IntoResponse, AppError> {

    let cliente = state.loja_service.adicionar_cliente(
        loja_uuid,
        p.nome,
        p.username,
        p.email,
        p.senha,
        p.celular,
        // p.telefone
    ).await?;

    Ok(Json(cliente))
}
