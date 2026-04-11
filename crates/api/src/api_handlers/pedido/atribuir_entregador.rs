use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    usecases::PedidoUsecase
};
use crate::api_handlers::{dto::AppError, AppState};

#[derive(Deserialize)]
pub struct AtribuirEntregadorRequest {
    pub entregador_uuid: Uuid,
}

/// PUT /api/pedidos/{pedido_uuid}/entregador
/// Atribui um entregador a um pedido. Requer que o pedido pertença à loja
/// informada no path.
pub async fn atribuir_entregador(
    State(state): State<Arc<AppState>>,
    Path((pedido_uuid, loja_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Json(req): Json<AtribuirEntregadorRequest>,
) -> Result<impl IntoResponse, AppError> {

    // Valida que o pedido existe e pertence à loja
    let pedidos = state.pedido_service.listar_por_loja(loja_uuid).await?;
    let pedido = pedidos.into_iter()
        .find(|p| p.uuid == pedido_uuid)
        .ok_or(AppError::NotFound("Pedido não encontrado ou não pertence a esta loja".to_string()))?;

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        usuario,
        pedido.loja_uuid,
    );

    usecase.atribuir_entregador(pedido_uuid, req.entregador_uuid).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// DELETE /api/pedidos/{pedido_uuid}/entregador
/// Remove o entregador de um pedido.
pub async fn remover_entregador(
    State(state): State<Arc<AppState>>,
    Path((pedido_uuid, loja_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

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
