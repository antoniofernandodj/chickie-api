use axum::{Extension, extract::{Path, State}};
use uuid::Uuid;
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;
use chickie_core::{models::Usuario, usecases::AdminUsecase, proto};

pub async fn buscar_config_pedido(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::ConfigPedido>, AppError> {
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
    let config = uc.buscar_config_pedido().await?;
    match config {
        Some(c) => Ok(Protobuf(c.to_proto())),
        None => Err(AppError::NotFound("Configuração de pedido não encontrada".to_string())),
    }
}
