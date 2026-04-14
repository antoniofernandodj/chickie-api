use axum::{Extension, extract::{Path, State}};
use uuid::Uuid;
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState};
use chickie_core::{models::{HorarioFuncionamento, Usuario}, usecases::AdminUsecase, proto};
use crate::handlers::protobuf::Protobuf;

pub async fn criar_ou_atualizar_horario(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::CriarOuAtualizarHorarioRequest>,
) -> Result<Protobuf<proto::GenericResponse>, AppError> {
    let mut horario = HorarioFuncionamento::new(
        loja_uuid,
        p.dia_semana,
        p.abertura.clone(),
        p.fechamento.clone(),
    ).map_err(|e| AppError::BadRequest(e))?;
    horario.ativo = p.ativo;

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
    uc.criar_ou_atualizar_horario(&horario).await?;
    Ok(Protobuf(proto::GenericResponse {
        message: "Horário salvo com sucesso".to_string(),
        success: true,
    }))
}
