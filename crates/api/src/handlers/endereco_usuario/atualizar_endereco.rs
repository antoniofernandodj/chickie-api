use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    proto
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;

pub async fn atualizar_endereco(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::EnderecoRequest>,
) -> Result<Protobuf<proto::Endereco>, AppError> {

    let cep = if p.cep.is_empty() { None } else { Some(p.cep.clone()) };
    let complemento = if p.complemento.is_empty() { None } else { Some(p.complemento.clone()) };
    let latitude = if p.latitude.is_empty() { None } else { Some(p.latitude.parse().unwrap_or_default()) };
    let longitude = if p.longitude.is_empty() { None } else { Some(p.longitude.parse().unwrap_or_default()) };

    let endereco = state.endereco_usuario_service.atualizar_endereco(
        uuid,
        usuario.uuid,
        cep,
        p.logradouro,
        p.numero,
        complemento,
        p.bairro,
        p.cidade,
        p.estado,
        latitude,
        longitude
    ).await?;

    Ok(Protobuf(endereco.to_proto()))
}
