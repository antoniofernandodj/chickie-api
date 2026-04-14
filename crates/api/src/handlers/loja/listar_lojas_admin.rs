use axum::extract::State;
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;
use chickie_core::proto;

pub async fn listar_lojas_admin(
    State(state): State<Arc<AppState>>,
) -> Result<Protobuf<proto::ListarLojasResponse>, AppError> {

    let lojas = state.loja_service.listar().await?;

    Ok(Protobuf(proto::ListarLojasResponse {
        lojas: lojas.into_iter().map(|l| l.to_proto()).collect(),
    }))
}
