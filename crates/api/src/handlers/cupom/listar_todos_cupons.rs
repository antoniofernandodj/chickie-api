use axum::{Extension, extract::State};
use std::sync::Arc;

use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};
use chickie_core::{models::Usuario, proto};
use chickie_core::ports::to_proto::ToProto;

pub async fn listar_todos_cupons(
    State(state): State<Arc<AppState>>,
    Extension(_): Extension<Usuario>,
) -> Result<Protobuf<proto::ListarCuponsResponse>, AppError> {

    let cupons = state.marketing_service.listar_todos_cupons().await?;

    let cupons_proto = cupons.iter().map(|c| c.to_proto()).collect();

    Ok(Protobuf(proto::ListarCuponsResponse { cupons: cupons_proto }))
}
