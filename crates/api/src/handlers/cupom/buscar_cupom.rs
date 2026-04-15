use axum::{Extension, extract::{Path, State}};
use uuid::Uuid;
use std::sync::Arc;

use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};
use chickie_core::{models::Usuario, proto};
use chickie_core::ports::to_proto::ToProto;

pub async fn buscar_cupom(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
) -> Result<Protobuf<proto::Cupom>, AppError> {

    let cupom = state.marketing_service.buscar_cupom(uuid).await?;

    Ok(Protobuf(cupom.to_proto()))
}
