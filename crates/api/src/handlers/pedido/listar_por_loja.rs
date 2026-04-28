use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::{EstadoDePedido, Usuario},
    usecases::PedidoUsecase,
};
use crate::handlers::{dto::AppError, AppState};

#[derive(Deserialize)]
pub struct FiltroStatus {
    pub status: Option<String>,
}

pub async fn listar_por_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Query(filtro): Query<FiltroStatus>,
) -> Result<impl IntoResponse, AppError> {

    let status = filtro
        .status
        .as_deref()
        .map(|s| EstadoDePedido::from_str(s).map_err(|e| AppError::BadRequest(e)))
        .transpose()?
        .unwrap_or(EstadoDePedido::Criado);

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        Some(usuario),
        loja_uuid,
    );

    let pedidos = usecase.listar_por_loja(status).await?;

    Ok(Json(pedidos))
}
