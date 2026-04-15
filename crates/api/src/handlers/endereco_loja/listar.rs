use axum::{Extension, extract::{Path, State}};
use uuid::Uuid;
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::{models::Usuario, usecases::AdminUsecase, proto};
use chickie_core::ports::to_proto::ToProto;

pub async fn listar_enderecos_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::ListarEnderecosResponse>, AppError> {

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

    let enderecos = uc.listar_enderecos().await?;
    
    let enderecos_proto = enderecos.iter().map(|e| e.to_proto()).collect();

    Ok(Protobuf(proto::ListarEnderecosResponse { enderecos: enderecos_proto }))
}
