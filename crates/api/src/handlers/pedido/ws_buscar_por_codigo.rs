use axum::{
    extract::{Path, State},
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use std::sync::Arc;
use tokio::time::Duration;

use crate::handlers::{AppState, dto::AppError};

pub async fn ws_buscar_por_codigo(
    State(state): State<Arc<AppState>>,
    Path(codigo): Path<String>,
    ws: WebSocketUpgrade,
) -> Result<Response, AppError> {
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, state, codigo)))
}

async fn handle_socket(
    mut socket: WebSocket,
    state: Arc<AppState>,
    codigo: String,
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
                match state.pedido_service.buscar_por_codigo(&codigo).await {
                    Ok(pedido) => {
                        if let Ok(json) = serde_json::to_string(&pedido) {
                            if json != ultimo_json {
                                if socket.send(Message::Text(json.clone().into())).await.is_err() {
                                    break;
                                }
                                ultimo_json = json;
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("ws_buscar_por_codigo erro codigo={}: {}", codigo, e);
                    }
                }
            }
        }
    }
}
