use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse
};
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::Usuario,
    usecases::PedidoUsecase
};
use crate::handlers::{dto::AppError, AppState};

pub async fn listar_por_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        usuario,
        loja_uuid,
    );

    let pedidos = usecase.listar_por_loja().await?;

    Ok(Json(pedidos))
}
