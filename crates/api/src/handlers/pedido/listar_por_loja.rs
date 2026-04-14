use axum::{Extension, extract::{Path, State}};
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::Usuario,
    usecases::PedidoUsecase,
    proto,
    ports::to_proto::ToProto,
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn listar_por_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::ListarPedidosResponse>, AppError> {

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        usuario,
        loja_uuid,
    );

    let pedidos = usecase.listar_por_loja().await?;

    Ok(Protobuf(proto::ListarPedidosResponse {
        pedidos: pedidos.into_iter().map(|p| p.to_proto()).collect(),
    }))
}
