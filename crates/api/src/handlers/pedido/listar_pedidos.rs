use axum::{
    extract::{Path, State},
};
use uuid::Uuid;

use std::sync::Arc;
use crate::{handlers::{AppState, dto::AppError, protobuf::Protobuf}};
use chickie_core::proto;


pub async fn listar_pedidos(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<Protobuf<proto::ListarPedidosResponse>, AppError> {

    let pedidos = state
        .pedido_repo
        .buscar_por_loja(loja_uuid)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let proto_pedidos: Vec<_> = pedidos.into_iter()
        .map(|p| p.to_proto())
        .collect();

    Ok(Protobuf(proto::ListarPedidosResponse {
        pedidos: proto_pedidos,
    }))
}


/*

use axum::{
    Json, extract::{Path, State}, response::IntoResponse
};
use uuid::Uuid;

use std::sync::Arc;
use crate::{handlers::dto::AppError};
use crate::handlers::AppState;



pub async fn listar_pedidos(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {

    let pedidos = state
        .pedido_repo
        .buscar_por_loja(loja_uuid)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(pedidos))
}

*/