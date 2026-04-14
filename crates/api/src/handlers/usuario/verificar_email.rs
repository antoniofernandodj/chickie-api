use axum::extract::State;
use std::sync::Arc;

use crate::handlers::dto::AppError;
use crate::handlers::{AppState, protobuf::Protobuf};
use chickie_core::proto;

pub async fn verificar_email(
    State(state): State<Arc<AppState>>,
    Protobuf(body): Protobuf<proto::VerificarEmailRequest>,
) -> Result<Protobuf<proto::DisponibilidadeResponse>, AppError> {
    let disponivel = state
        .usuario_service
        .verificar_email_disponivel(&body.email)
        .await
        .map_err(|e| AppError::Internal(e))?;

    Ok(Protobuf(proto::DisponibilidadeResponse { disponivel }))
}
