use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse
};
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::Usuario,
    ports::PedidoRepositoryPort,
    usecases::PedidoUsecase,
};
use crate::handlers::{dto::AppError, AppState};

pub async fn cancelar_pedido(
    State(state): State<Arc<AppState>>,
    Path(pedido_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

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

    let pedido = usecase.cancelar_pedido(pedido_uuid).await?;

    Ok(Json(serde_json::json!({
        "uuid": pedido.uuid,
        "status": pedido.status,
    })))
}
