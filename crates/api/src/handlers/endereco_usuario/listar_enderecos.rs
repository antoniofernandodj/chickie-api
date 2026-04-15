use axum::{Extension, extract::State};
use std::sync::Arc;

use chickie_core::models::Usuario;
use chickie_core::proto;
use chickie_core::ports::to_proto::ToProto;
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn listar_enderecos(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::ListarEnderecosResponse>, AppError> {

    let enderecos = state.endereco_usuario_service
        .listar_enderecos(usuario.uuid)
        .await?;

    let enderecos_proto = enderecos.iter().map(|e| e.to_proto()).collect();

    Ok(Protobuf(proto::ListarEnderecosResponse { enderecos: enderecos_proto }))
}
