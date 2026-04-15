use axum::{
    Extension, extract::{Path, State}
};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    proto
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;

/// GET /proto/pedidos/{pedido_uuid}/com-entregador
/// Busca um pedido com informações do entregador vinculado.
pub async fn buscar_pedido_com_entregador(
    State(state): State<Arc<AppState>>,
    Path(pedido_uuid): Path<Uuid>,
    Extension(_usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::PedidoComEntregadorResponse>, AppError> {

    let pedido = state.pedido_service
        .buscar_pedido_com_entregador(pedido_uuid)
        .await?;

    let pedido_proto = pedido.pedido.to_proto();

    Ok(Protobuf(proto::PedidoComEntregadorResponse {
        pedido: Some(pedido_proto),
        entregador_nome: pedido.entregador_nome.unwrap_or_default(),
        veiculo: pedido.veiculo.unwrap_or_default(),
        placa: pedido.placa.unwrap_or_default(),
    }))
}


/*

use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse
};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::models::Usuario;
use crate::handlers::{dto::AppError, AppState};

/// GET /api/pedidos/{pedido_uuid}/com-entregador
/// Busca um pedido com informações do entregador vinculado.
pub async fn buscar_pedido_com_entregador(
    State(state): State<Arc<AppState>>,
    Path(pedido_uuid): Path<Uuid>,
    Extension(_usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let pedido = state.pedido_service
        .buscar_pedido_com_entregador(pedido_uuid)
        .await?;

    Ok(Json(pedido))
}

*/
