use axum::{Extension, extract::{Path, State}};
use uuid::Uuid;
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState};
use chickie_core::{models::Usuario, usecases::AdminUsecase, proto};
use crate::handlers::protobuf::Protobuf;

pub async fn atualizar_ingrediente(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::AtualizarIngredienteRequest>,
) -> Result<Protobuf<proto::GenericResponse>, AppError> {
    let uc = AdminUsecase::new(state.ingrediente_service.clone(), state.horario_funcionamento_service.clone(), state.config_pedido_service.clone(), state.funcionario_service.clone(), state.entregador_service.clone(), state.marketing_service.clone(), state.endereco_loja_service.clone(), usuario, loja_uuid);
    uc.atualizar_ingrediente(
        uuid,
        p.nome,
        if p.unidade_medida.is_empty() { None } else { Some(p.unidade_medida) },
        p.quantidade.parse::<f64>().map_err(|e| AppError::BadRequest(format!("Quantidade inválida: {}", e)))?,
        p.preco_unitario,
    ).await?;
    Ok(Protobuf(proto::GenericResponse {
        message: "Ingrediente atualizado com sucesso".to_string(),
        success: true,
    }))
}
