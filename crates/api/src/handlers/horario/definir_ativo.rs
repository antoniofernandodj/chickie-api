use axum::{Extension, extract::{Path, State}};
use uuid::Uuid;
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState};
use chickie_core::{models::Usuario, usecases::AdminUsecase, proto};
use crate::handlers::protobuf::Protobuf;

pub async fn definir_ativo(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, dia_semana)): Path<(Uuid, i32)>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::DefinirAtivoRequest>,
) -> Result<Protobuf<proto::GenericResponse>, AppError> {
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
    uc.definir_horario_ativo(dia_semana, p.ativo).await?;
    Ok(Protobuf(proto::GenericResponse {
        message: "Status do horário atualizado com sucesso".to_string(),
        success: true,
    }))
}
