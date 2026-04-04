use axum::{Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::{AppState, auth::AdminPermission, dto::AppError},
};

#[derive(Deserialize)]
pub struct AdicionarEntregadorRequest {
    pub nome: String,
    pub username: String,
    pub email: String,
    pub senha: String,
    pub celular: String,
    pub veiculo: Option<String>,
    pub placa: Option<String>,
}

pub async fn adicionar_entregador(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    AdminPermission(_): AdminPermission,
    Json(p): Json<AdicionarEntregadorRequest>,
) -> Result<impl IntoResponse, AppError> {

    let entregador = state.loja_service.adicionar_entregador(
        loja_uuid,
        p.nome,
        p.username,
        p.email,
        p.senha,
        p.celular,
        p.veiculo,
        p.placa
    ).await?;

    Ok(Json(entregador))
}
