use std::sync::Arc;

use axum::{Extension, extract::{Path, State}};
use uuid::Uuid;

use chickie_core::{models::Usuario, proto};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn atualizar_disponibilidade(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, adicional_uuid)): Path<(Uuid, Uuid)>,
    Extension(_): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::AtualizarDisponibilidadeRequest>,
) -> Result<Protobuf<proto::GenericResponse>, AppError> {

    state.catalogo_service.atualizar_disponibilidade(adicional_uuid, loja_uuid, p.disponivel).await?;

    Ok(Protobuf(proto::GenericResponse {
        message: "Disponibilidade do adicional atualizada com sucesso".to_string(),
        success: true,
    }))
}
