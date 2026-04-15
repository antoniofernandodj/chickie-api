use axum::{
    extract::State
};
use std::sync::Arc;
use crate::handlers::{AppState, auth::AdminPermission, dto::AppError, protobuf::Protobuf};
use chickie_core::proto;
use chickie_core::ports::to_proto::ToProto;

/// Lista todas as lojas criadas pelo admin autenticado
pub async fn listar_minhas_lojas(
    State(state): State<Arc<AppState>>,
    AdminPermission(usuario): AdminPermission,
) -> Result<Protobuf<proto::ListarLojasResponse>, AppError> {

    let lojas = state
        .loja_service
        .listar_por_criador(usuario.uuid)
        .await?;

    let lojas_proto = lojas.iter().map(|l| l.to_proto()).collect();

    Ok(Protobuf(proto::ListarLojasResponse { lojas: lojas_proto }))
}
