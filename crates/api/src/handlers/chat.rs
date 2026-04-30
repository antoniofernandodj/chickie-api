use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Query},
    response::{IntoResponse, Response},
    Json,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use uuid::Uuid;
use crate::handlers::{AppState, validar_token, dto::AppError};
use chickie_core::models::{CreateMensagemRequest, Usuario, MensagemChat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WsChatQuery {
    pub token: String,
}

pub async fn ws_chat_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Query(query): Query<WsChatQuery>,
) -> Response {
    match validar_token(&query.token, &state).await {
        Ok(usuario) => ws.on_upgrade(move |socket| handle_chat_socket(socket, state, usuario)),
        Err(e) => e.into_response(),
    }
}

async fn handle_chat_socket(socket: WebSocket, state: Arc<AppState>, usuario: Usuario) {
    let (mut sender, mut receiver) = socket.split();
    let usuario_uuid = usuario.uuid;

    // 1. Identificar canais para se inscrever
    let mut canais = vec![format!("chat:usuario:{}", usuario_uuid)];
    
    let lojas_associadas = get_lojas_associadas(&state, &usuario).await;
    for l_uuid in lojas_associadas {
        canais.push(format!("chat:loja:{}", l_uuid));
    }

    // 2. Iniciar Subscriber Redis
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Falha ao conectar no Redis: {}", e);
            return;
        },
    };
    
    let mut pubsub_conn = match client.get_async_pubsub().await {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Falha ao obter conexão pubsub do Redis: {}", e);
            return;
        },
    };
    
    for canal in canais {
        let _ = pubsub_conn.subscribe(canal).await;
    }

    // 3. Task para ler do Redis e enviar para o WebSocket
    // Movemos a conexão pubsub_conn para dentro da task para evitar problemas de lifetime
    let sender_task = tokio::spawn(async move {
        let mut pubsub_stream = pubsub_conn.on_message();
        while let Some(msg) = pubsub_stream.next().await {
            let payload: String = match msg.get_payload() {
                Ok(p) => p,
                Err(_) => continue,
            };
            if sender.send(Message::Text(payload.into())).await.is_err() {
                break;
            }
        }
    });

    // 4. Loop principal: ler do WebSocket e processar (enviar mensagens)
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            if let Ok(req) = serde_json::from_str::<CreateMensagemRequest>(&text) {
                // Envia mensagem via Usecase (persiste e publica no Redis)
                let _ = state.chat_usecase.enviar_mensagem(req, usuario_uuid).await;
            }
        }
    }

    sender_task.abort();
}

async fn get_lojas_associadas(state: &AppState, usuario: &Usuario) -> Vec<Uuid> {
    let mut lojas = Vec::new();
    let usuario_uuid = usuario.uuid;

    // Se for administrador, buscar lojas criadas por ele
    if usuario.classe == "administrador" || usuario.classe == "owner" {
        let query_admin = "SELECT uuid FROM lojas WHERE criado_por = $1";
        if let Ok(rows) = sqlx::query_as::<_, (Uuid,)>(query_admin).bind(usuario_uuid).fetch_all(&*state.db).await {
            for (id,) in rows { lojas.push(id); }
        }
    }

    // Buscar onde é funcionário
    let query_func = "SELECT loja_uuid FROM funcionarios WHERE usuario_uuid = $1";
    if let Ok(rows) = sqlx::query_as::<_, (Uuid,)>(query_func).bind(usuario_uuid).fetch_all(&*state.db).await {
        for (id,) in rows { lojas.push(id); }
    }

    // Buscar onde é entregador
    let query_entregador = "SELECT loja_uuid FROM entregadores WHERE usuario_uuid = $1";
    if let Ok(rows) = sqlx::query_as::<_, (Uuid,)>(query_entregador).bind(usuario_uuid).fetch_all(&*state.db).await {
        for (id,) in rows { lojas.push(id); }
    }

    lojas
}

// --- Endpoints HTTP para Histórico ---

pub async fn listar_historico_pedido(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(pedido_uuid): axum::extract::Path<Uuid>,
) -> Result<Json<Vec<MensagemChat>>, AppError> {
    let mensagens = state.chat_usecase.listar_historico_pedido(pedido_uuid).await?;
    Ok(Json(mensagens))
}

pub async fn listar_historico_loja_usuario(
    State(state): State<Arc<AppState>>,
    axum::extract::Path((loja_uuid, usuario_uuid)): axum::extract::Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<MensagemChat>>, AppError> {
    let mensagens = state.chat_usecase.listar_historico_loja_usuario(loja_uuid, usuario_uuid).await?;
    Ok(Json(mensagens))
}

pub async fn marcar_lida(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(mensagem_uuid): axum::extract::Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.chat_usecase.marcar_lida(mensagem_uuid).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}
