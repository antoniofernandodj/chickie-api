use axum::{
    Extension, extract::{Path, State}
};
use uuid::Uuid;
use std::sync::Arc;
use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};
use chickie_core::{models::Usuario, proto};
use chickie_core::ports::to_proto::ToProto;

pub async fn validar_cupom(
    Extension(_): Extension<Usuario>,
    State(state): State<Arc<AppState>>,
    Path((codigo, loja_uuid)): Path<(String, Uuid)>,
) -> Result<Protobuf<proto::Cupom>, AppError> {

    let cupom = state.cupom_repo
        .buscar_por_codigo(&codigo, loja_uuid)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Cupom não encontrado".into()))?;

    Ok(Protobuf(cupom.to_proto()))
}
