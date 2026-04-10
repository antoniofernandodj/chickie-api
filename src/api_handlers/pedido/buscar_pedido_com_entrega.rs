use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse
};
use serde::Serialize;
use uuid::Uuid;
use std::sync::Arc;

use crate::{
    api::{dto::AppError, AppState},
    models::Usuario,
    services::PedidoComEntrega,
    usecases::PedidoUsecase
};

#[derive(Serialize)]
pub struct PedidoComEntregaResponse {
    pub pedido: crate::models::Pedido,
    pub endereco_entrega: Option<crate::models::EnderecoEntrega>,
}

pub async fn buscar_pedido_com_entrega(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, pedido_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        usuario,
        loja_uuid,
    );

    let resultado: PedidoComEntrega = usecase.buscar_pedido_com_entrega(pedido_uuid).await?;

    Ok(Json(PedidoComEntregaResponse {
        pedido: resultado.pedido,
        endereco_entrega: resultado.endereco_entrega,
    }))
}
