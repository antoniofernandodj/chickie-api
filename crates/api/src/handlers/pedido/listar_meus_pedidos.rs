use axum::{Extension, extract::State};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    ports::to_proto::ToProto,
    usecases::PedidoUsecase,
    proto,
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

/// GET /api/pedidos/meus
/// Lista todos os pedidos do usuário autenticado.
pub async fn listar_meus_pedidos(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::ListarPedidosResponse>, AppError> {

    // Usa uma loja_uuid fictícia (nil) pois o usecase só precisa do
    // usuario.uuid para listar_por_usuario. O campo loja_uuid do usecase
    // é ignorado neste método.
    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        usuario,
        Uuid::nil(),
    );

    let pedidos = usecase.listar_por_usuario().await?;

    Ok(Protobuf(proto::ListarPedidosResponse {
        pedidos: pedidos.into_iter().map(|p| p.to_proto()).collect(),
    }))
}
