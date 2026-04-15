use axum::{Extension, extract::State};
use std::sync::Arc;

use chickie_core::{
    models::Usuario,
    proto
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;

pub async fn criar_endereco(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::EnderecoRequest>,
) -> Result<Protobuf<proto::Endereco>, AppError> {

    let cep = if p.cep.is_empty() { None } else { Some(p.cep.clone()) };
    let complemento = if p.complemento.is_empty() { None } else { Some(p.complemento.clone()) };

    let endereco = state.endereco_usuario_service.criar_endereco(
        usuario.uuid,
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
use axum::{Extension, Json, extract::State, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;

use chickie_core::models::Usuario;
use crate::handlers::{dto::AppError, AppState};

#[derive(Deserialize)]
pub struct CreateEnderecoUsuarioRequest {
    pub cep: Option<String>,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
}

pub async fn criar_endereco(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<CreateEnderecoUsuarioRequest>,
) -> Result<impl IntoResponse, AppError> {

    let endereco = state.endereco_usuario_service.criar_endereco(
        usuario.uuid,
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
