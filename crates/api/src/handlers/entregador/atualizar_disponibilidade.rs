use axum::{Extension, extract::{Path, State}, http::StatusCode};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    usecases::AdminUsecase,
};
use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};
use chickie_core::proto;

pub async fn atualizar_disponibilidade_entregador(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, entregador_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::AtualizarDisponibilidadeRequest>,
) -> Result<StatusCode, AppError> {

    let admin_usecase = AdminUsecase::new(
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

    admin_usecase
        .definir_entregador_disponivel(entregador_uuid, p.disponivel)
        .await
        .map_err(|e| AppError::BadRequest(e))?;

    Ok(StatusCode::NO_CONTENT)
}
