use axum::extract::State;
use std::sync::Arc;

use crate::handlers::dto::AppError;
use crate::handlers::{AppState, protobuf::Protobuf};
use chickie_core::proto;

pub async fn verificar_username(
    State(state): State<Arc<AppState>>,
    Protobuf(body): Protobuf<proto::VerificarUsernameRequest>,
) -> Result<Protobuf<proto::DisponibilidadeResponse>, AppError> {
    let disponivel = state
        .usuario_service
        .verificar_username_disponivel(&body.username)
        .await
        .map_err(|e| AppError::Internal(e))?;

    Ok(Protobuf(proto::DisponibilidadeResponse { disponivel }))
}
