use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    proto
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;

pub async fn criar_para_pedido(
    State(state): State<Arc<AppState>>,
    Path((pedido_uuid, loja_uuid)): Path<(Uuid, Uuid)>,
    Extension(_): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::EnderecoRequest>,
) -> Result<Protobuf<proto::EnderecoEntrega>, AppError> {

    let cep = if p.cep.is_empty() { None } else { Some(p.cep.clone()) };
    let complemento = if p.complemento.is_empty() { None } else { Some(p.complemento.clone()) };

    let endereco = state.endereco_entrega_service.criar_para_pedido(
        pedido_uuid,
        loja_uuid,
        cep,
        p.logradouro,
        p.numero,
        complemento,
        p.bairro,
        p.cidade,
        p.estado
    ).await?;

    Ok(Protobuf(endereco.to_proto()))
}

/*
use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::models::Usuario;
use crate::handlers::{dto::AppError, AppState};

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
    Extension(_): Extension<Usuario>,
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
*/
