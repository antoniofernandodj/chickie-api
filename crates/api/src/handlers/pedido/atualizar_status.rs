use axum::{
    Extension, extract::{Path, State}
};
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::{EstadoDePedido, Usuario},
    usecases::PedidoUsecase,
    proto
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn atualizar_status(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, pedido_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::AtualizarStatusPedidoRequest>,
) -> Result<Protobuf<proto::PedidoStatusResponse>, AppError> {

    let novo_status = EstadoDePedido::from_str(&p.novo_status)
        .map_err(|e| AppError::BadRequest(e))?;

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        usuario,
        loja_uuid,
    );

    let pedido = usecase.atualizar_status_pedido(pedido_uuid, novo_status).await?;

    let transicoes: Vec<String> = pedido.status.transicoes_permitidas()
        .iter().map(|s| s.as_str().to_string()).collect();

    Ok(Protobuf(proto::PedidoStatusResponse {
        uuid: pedido.uuid.to_string(),
        status: pedido.status.as_str().to_string(),
        transicoes_permitidas: transicoes,
    }))
}

/*
use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse
};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::{EstadoDePedido, Usuario},
    usecases::PedidoUsecase
};
use crate::handlers::{dto::AppError, AppState};

#[derive(Deserialize)]
pub struct AtualizarStatusRequest {
    pub novo_status: String,
}

pub async fn atualizar_status(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, pedido_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<AtualizarStatusRequest>,
) -> Result<impl IntoResponse, AppError> {

    let novo_status = EstadoDePedido::from_str(&p.novo_status)
        .map_err(|e| AppError::BadRequest(e))?;

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        usuario,
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
*/
