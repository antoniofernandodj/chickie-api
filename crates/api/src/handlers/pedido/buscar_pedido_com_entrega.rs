use axum::{
    Extension, extract::{Path, State}
};
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::Usuario,
    usecases::PedidoUsecase,
    proto
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;

pub async fn buscar_pedido_com_entrega(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, pedido_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::PedidoComEntregaResponse>, AppError> {

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        usuario,
        loja_uuid,
    );

    let resultado = usecase.buscar_pedido_com_entrega(pedido_uuid).await?;

    let pedido_proto = resultado.pedido.to_proto();
    let endereco_proto = resultado.endereco_entrega.map(|e| e.to_proto());

    Ok(Protobuf(proto::PedidoComEntregaResponse {
        pedido: Some(pedido_proto),
        endereco_entrega: endereco_proto,
    }))
}


/*

use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse
};
use serde::Serialize;
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::Usuario,
    services::PedidoComEntrega,
    usecases::PedidoUsecase
};
use crate::handlers::{dto::AppError, AppState};

#[derive(Serialize)]
pub struct PedidoComEntregaResponse {
    pub pedido: chickie_core::models::Pedido,
    pub endereco_entrega: Option<chickie_core::models::EnderecoEntrega>,
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

*/
