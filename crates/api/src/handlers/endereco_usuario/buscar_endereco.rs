use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::models::Usuario;
use chickie_core::proto;
use chickie_core::ports::to_proto::ToProto;
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn buscar_endereco(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::Endereco>, AppError> {

    let endereco = state.endereco_usuario_service
        .buscar_endereco(uuid, usuario.uuid)
        .await?;

    let msg = "Endereço não encontrado";
    match endereco {
        Some(e) => Ok(Protobuf(e.to_proto())),
        None => Err(AppError::NotFound(msg.to_string())),
    }
}
