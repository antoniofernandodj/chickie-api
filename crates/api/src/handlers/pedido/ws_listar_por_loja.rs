use axum::{
    extract::{Path, Query, State},
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::time::Duration;
use uuid::Uuid;

use chickie_core::models::EstadoDePedido;
use crate::handlers::{AppState, validar_token, dto::AppError};

#[derive(Deserialize)]
pub struct WsFiltro {
    pub status: Option<String>,
    pub token: String,
}

pub async fn ws_listar_por_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Query(filtro): Query<WsFiltro>,
    ws: WebSocketUpgrade,
) -> Result<Response, AppError> {
    validar_token(&filtro.token, &state).await?;

    let status = filtro
        .status
        .as_deref()
        .map(|s| EstadoDePedido::from_str(s).map_err(AppError::BadRequest))
        .transpose()?
        .unwrap_or(EstadoDePedido::Criado);

    Ok(
        ws.on_upgrade(
            move |socket| handle_socket(
                socket, state, loja_uuid, status
            )
        )
    )
}

async fn handle_socket(
    mut socket: WebSocket,
    state: Arc<AppState>,
    loja_uuid: Uuid,
    status: EstadoDePedido,
) {
    let mut ultimo_json = String::new();
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            msg = socket.recv() => {
                match msg {
                    None | Some(Ok(Message::Close(_))) | Some(Err(_)) => break,
                    _ => {}
                }
            }
            _ = interval.tick() => {
                match state.pedido_service.listar_por_loja(loja_uuid, status.clone()).await {
                    Ok(pedidos) => {
                        if let Ok(json) = serde_json::to_string(&pedidos) {
                            if json != ultimo_json {
                                if socket.send(Message::Text(json.clone().into())).await.is_err() {
                                    break;
                                }
                                ultimo_json = json;
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("ws_listar_por_loja erro loja={}: {}", loja_uuid, e);
                    }
                }
            }
        }
    }
}
