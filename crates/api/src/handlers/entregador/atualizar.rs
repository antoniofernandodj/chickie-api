use axum::{Extension, extract::{Path, State}, http::StatusCode};
use std::sync::Arc;
use uuid::Uuid;

use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::{models::Usuario, usecases::AdminUsecase, proto};

pub async fn atualizar_entregador(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::AtualizarEntregadorRequest>,
) -> Result<StatusCode, AppError> {

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
    let usuario_uuid = Uuid::parse_str(&p.usuario_uuid)
        .map_err(|e| AppError::BadRequest(format!("Invalid usuario_uuid: {}", e)))?;
    let nome = if p.nome.is_empty() { None } else { Some(p.nome.clone()) };
    let celular = if p.celular.is_empty() { None } else { Some(p.celular.clone()) };
    let veiculo = if p.veiculo.is_empty() { None } else { Some(p.veiculo.clone()) };
    let placa = if p.placa.is_empty() { None } else { Some(p.placa.clone()) };
    uc.atualizar_entregador(uuid, usuario_uuid, nome, celular, veiculo, placa).await?;

    Ok(StatusCode::NO_CONTENT)
}

