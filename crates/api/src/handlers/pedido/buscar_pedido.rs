use axum::{
    extract::{Path, State},
};
use uuid::Uuid;

use std::sync::Arc;
use crate::{handlers::{AppState, dto::AppError, protobuf::Protobuf}};
use chickie_core::proto;
use chickie_core::ports::to_proto::ToProto;


pub async fn buscar_pedido(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, uuid)): Path<(Uuid, Uuid)>,
) -> Result<Protobuf<proto::Pedido>, AppError> {

    let pedido = state
        .pedido_repo
        .buscar_completo(uuid, loja_uuid)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Pedido não encontrado".to_string()))?;

    Ok(Protobuf(pedido.to_proto()))
}


/*

use axum::{
    Json, extract::{Path, State}, response::{IntoResponse}
};
use uuid::Uuid;

use std::sync::Arc;
use crate::{handlers::dto::AppError};
use crate::handlers::AppState;


pub async fn buscar_pedido(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {

    let pedido = state
        .pedido_repo
        .buscar_completo(uuid, loja_uuid)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Pedido não encontrado".to_string()))?;

    Ok(Json(pedido))
}

*/