use axum::extract::State;
use std::sync::Arc;

use crate::handlers::dto::AppError;
use crate::handlers::{AppState, protobuf::Protobuf};
use chickie_core::proto;

pub async fn verificar_celular(
    State(state): State<Arc<AppState>>,
    Protobuf(body): Protobuf<proto::VerificarCelularRequest>,
) -> Result<Protobuf<proto::DisponibilidadeResponse>, AppError> {
    let disponivel = state
        .usuario_service
        .verificar_celular_disponivel(&body.celular)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Protobuf(proto::DisponibilidadeResponse { disponivel }))
}
