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
use crate::handlers::{dto::AppError, AppState};

#[derive(Deserialize)]
pub struct AtribuirEntregadorRequest {
    pub entregador_uuid: Uuid,
}

/// PUT /api/pedidos/{pedido_uuid}/entregador
pub async fn atribuir_entregador(
    State(state): State<Arc<AppState>>,
    Path((pedido_uuid, loja_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Json(req): Json<AtribuirEntregadorRequest>,
) -> Result<impl IntoResponse, AppError> {

    let pedido = state.pedido_service.buscar_por_uuid(pedido_uuid).await?;
    if pedido.loja_uuid != loja_uuid {
        return Err(AppError::NotFound("Pedido não encontrado ou não pertence a esta loja".to_string()));
    }

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        Some(usuario),
        loja_uuid,
    );

    usecase.atribuir_entregador(pedido_uuid, req.entregador_uuid).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// DELETE /api/pedidos/{pedido_uuid}/entregador
pub async fn remover_entregador(
    State(state): State<Arc<AppState>>,
    Path((pedido_uuid, loja_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let pedido = state.pedido_service.buscar_por_uuid(pedido_uuid).await?;
    if pedido.loja_uuid != loja_uuid {
        return Err(AppError::NotFound("Pedido não encontrado ou não pertence a esta loja".to_string()));
    }

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        Some(usuario),
        loja_uuid,
    );

    usecase.remover_entregador(pedido_uuid).await?;

    Ok(StatusCode::NO_CONTENT)
}
