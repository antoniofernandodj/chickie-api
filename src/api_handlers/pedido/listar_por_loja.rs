use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse
};
use uuid::Uuid;
use std::sync::Arc;

use crate::{
    api::{dto::AppError, AppState},
    models::Usuario,
    usecases::PedidoUsecase
};

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
