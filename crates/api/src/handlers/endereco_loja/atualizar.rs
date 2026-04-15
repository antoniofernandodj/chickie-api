use axum::{Extension, extract::{Path, State}, http::StatusCode};
use std::sync::Arc;
use uuid::Uuid;

use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::{models::{EnderecoLoja, Usuario}, usecases::AdminUsecase, proto};
use rust_decimal::Decimal;

pub async fn atualizar_endereco_loja(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, endereco_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::EnderecoRequest>,
) -> Result<StatusCode, AppError> {
    let cep = if p.cep.is_empty() { None } else { Some(p.cep.clone()) };
    let complemento = if p.complemento.is_empty() { None } else { Some(p.complemento.clone()) };
    let latitude = if p.latitude.is_empty() { None } else { Some(p.latitude.parse::<Decimal>().unwrap_or_default()) };
    let longitude = if p.longitude.is_empty() { None } else { Some(p.longitude.parse::<Decimal>().unwrap_or_default()) };

    let endereco = EnderecoLoja {
        uuid: endereco_uuid,
        loja_uuid,
        cep,
        logradouro: p.logradouro,
        numero: p.numero,
        complemento,
        bairro: p.bairro,
        cidade: p.cidade,
        estado: p.estado,
        latitude,
        longitude,
    };

    let uc = AdminUsecase::new(
        state.ingrediente_service.clone(),
        state.horario_funcionamento_service.clone(),
        state.config_pedido_service.clone(),
        state.funcionario_service.clone(),
        state.entregador_service.clone(),
        state.marketing_service.clone(),
        state.endereco_loja_service.clone(),
        usuario,
        loja_uuid,
    );
    uc.atualizar_endereco(endereco).await?;
    Ok(StatusCode::NO_CONTENT)
}
