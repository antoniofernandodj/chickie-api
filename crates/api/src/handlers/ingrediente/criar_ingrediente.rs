use axum::{Extension, extract::{Path, State}};
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::Usuario,
    ports::to_proto::ToProto,
    usecases::AdminUsecase,
    proto,
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn criar_ingrediente(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::CriarIngredienteRequest>,
) -> Result<Protobuf<proto::Ingrediente>, AppError> {
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
    let ingrediente = uc.criar_ingrediente(
        p.nome,
        if p.unidade_medida.is_empty() { None } else { Some(p.unidade_medida) },
        p.preco_unitario,
    ).await?;
    Ok(Protobuf(ingrediente.to_proto()))
}
