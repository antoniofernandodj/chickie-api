use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    proto,
};

use crate::{
    handlers::{AppState, dto::AppError, protobuf::Protobuf},
};

pub async fn atualizar_disponibilidade_produto(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, produto_uuid)): Path<(Uuid, Uuid)>,
    Extension(_): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::AtualizarDisponibilidadeRequest>,
) -> Result<Protobuf<proto::GenericResponse>, AppError> {

    state.catalogo_service.atualizar_disponibilidade_produto(produto_uuid, loja_uuid, p.disponivel).await?;

    Ok(Protobuf(proto::GenericResponse {
        message: "Disponibilidade do produto atualizada com sucesso".to_string(),
        success: true,
    }))
}
