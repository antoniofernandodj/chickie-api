use axum::{
    Extension, extract::{Path, State}, http::StatusCode
};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    usecases::PedidoUsecase,
    proto
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

/// PUT /proto/pedidos/{pedido_uuid}/entregador/{loja_uuid}
/// Atribui um entregador a um pedido. Requer que o pedido pertença à loja
/// informada no path.
pub async fn atribuir_entregador(
    State(state): State<Arc<AppState>>,
    Path((pedido_uuid, loja_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(req): Protobuf<proto::AtribuirEntregadorRequest>,
) -> Result<StatusCode, AppError> {

    // Valida que o pedido existe e pertence à loja
    let pedidos = state.pedido_service.listar_por_loja(loja_uuid).await?;
    let pedido = pedidos.into_iter()
        .find(|p| p.uuid == pedido_uuid)
        .ok_or(AppError::NotFound("Pedido não encontrado ou não pertence a esta loja".to_string()))?;

    let entregador_uuid = Uuid::parse_str(&req.entregador_uuid)
        .map_err(|e| AppError::BadRequest(format!("Invalid entregador_uuid: {}", e)))?;

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        usuario,
        pedido.loja_uuid,
    );

    usecase.atribuir_entregador(pedido_uuid, entregador_uuid).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn remover_entregador(
    State(state): State<Arc<AppState>>,
    Path((pedido_uuid, loja_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
) -> Result<StatusCode, AppError> {

    // Valida que o pedido existe e pertence à loja
    let pedidos = state.pedido_service.listar_por_loja(loja_uuid).await?;
    let _pedido = pedidos.into_iter()
        .find(|p| p.uuid == pedido_uuid)
        .ok_or(AppError::NotFound("Pedido não encontrado ou não pertence a esta loja".to_string()))?;

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        usuario,
        loja_uuid,
    );

    usecase.remover_entregador(pedido_uuid).await?;

    Ok(StatusCode::NO_CONTENT)
}
