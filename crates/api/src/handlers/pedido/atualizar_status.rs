use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse
};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::{EstadoDePedido, Usuario},
    ports::PedidoRepositoryPort,
    usecases::PedidoUsecase
};
use crate::handlers::{dto::AppError, AppState};

#[derive(Deserialize)]
pub struct AtualizarStatusRequest {
    pub novo_status: String,
}

pub async fn atualizar_status(
    State(state): State<Arc<AppState>>,
    Path(pedido_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<AtualizarStatusRequest>,
) -> Result<impl IntoResponse, AppError> {

    let novo_status = EstadoDePedido::from_str(&p.novo_status)
        .map_err(|e| AppError::BadRequest(e))?;

    let loja_uuid = state.pedido_repo
        .buscar_por_uuid(pedido_uuid).await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Pedido não encontrado".into()))?
        .loja_uuid;

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        Some(usuario),
        loja_uuid,
    );

    let pedido = usecase.atualizar_status_pedido(pedido_uuid, novo_status).await?;

    Ok(Json(serde_json::json!({
        "uuid": pedido.uuid,
        "status": pedido.status.as_str(),
        "transicoes_permitidas": pedido.status.transicoes_permitidas()
            .iter().map(|s| s.as_str()).collect::<Vec<_>>()
    })))
}
