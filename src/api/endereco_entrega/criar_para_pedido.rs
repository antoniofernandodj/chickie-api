use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::{dto::AppError, AppState},
    models::Usuario
};

#[derive(Deserialize)]
pub struct CreateEnderecoEntregaRequest {
    pub cep: Option<String>,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
}

pub async fn criar_para_pedido(
    State(state): State<Arc<AppState>>,
    Path((pedido_uuid, loja_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<CreateEnderecoEntregaRequest>,
) -> Result<impl IntoResponse, AppError> {

    let endereco = state.endereco_entrega_service.criar_para_pedido(
        pedido_uuid,
        loja_uuid,
        p.cep,
        p.logradouro,
        p.numero,
        p.complemento,
        p.bairro,
        p.cidade,
        p.estado
    ).await?;

    Ok(Json(endereco))
}
