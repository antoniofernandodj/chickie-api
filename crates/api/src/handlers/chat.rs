use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Query},
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::handlers::{AppState, validar_token, dto::AppError};
use chickie_core::models::{CreateMensagemRequest, Usuario, MensagemChat};
use serde::Deserialize;
use futures::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};


#[derive(Deserialize)]
pub struct WsChatQuery {
    pub token: String,
}


pub struct ChatSession {
    state: Arc<AppState>,
    usuario: Usuario,
    sender: SplitSink<WebSocket, Message>,
    receiver: SplitStream<WebSocket>,
}

impl ChatSession {
    pub fn new(socket: WebSocket, state: Arc<AppState>, usuario: Usuario) -> Self {
        let (sender, receiver) = socket.split();
        Self { state, usuario, sender, receiver }
    }

    /// Método principal que inicia os loops de leitura e escrita
    pub async fn start(self) {
        // Busca canais que o usuário tem permissão para ouvir
        let canais = self.usuario.buscar_canais_notificacao(&self.state.db).await;
        
        // Inicializa conexão PubSub do Redis
        let mut pubsub = match self.setup_redis_pubsub(canais).await {
            Ok(ps) => ps,
            Err(_) => return,
        };

        // Extract receiver before moving sender
        let mut receiver = self.receiver;
        
        // TASK OUTBOUND: Redis -> WebSocket
        let mut sender = self.sender;
        let sender_task = tokio::spawn(async move {
            let mut stream = pubsub.on_message();
            while let Some(msg) = stream.next().await {
                if let Ok(payload) = msg.get_payload::<String>() {
                    if sender.send(Message::Text(payload.into())).await.is_err() {
                        break; 
                    }
                }
            }
        });

        // LOOP INBOUND: WebSocket -> Banco/Redis (Processamento de mensagens enviadas)
        // Use the extracted receiver directly instead of self.receiver
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(req) = serde_json::from_str::<CreateMensagemRequest>(&text) {
                    // Delega para o Usecase persistir e publicar
                    let _ = self.state.chat_usecase
                        .enviar_mensagem(req, self.usuario.uuid)
                        .await;
                }
            }
        }

        // Ao sair do loop de entrada, encerra a task de saída
        sender_task.abort();
    }


    async fn setup_redis_pubsub(&self, canais: Vec<String>) -> Result<redis::aio::PubSub, ()> {
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

        let client = redis::Client::open(redis_url).map_err(|e| {
            tracing::error!("Erro Redis Client: {}", e);
        })?;

        let mut pubsub = client.get_async_pubsub().await.map_err(|e| {
            tracing::error!("Erro ao abrir conexão PubSub: {}", e);
        })?;

        for canal in canais {
            let _ = pubsub.subscribe(canal).await;
        }

        Ok(pubsub)
    }
}



pub async fn ws_chat_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Query(query): Query<WsChatQuery>,
) -> Response {
    match validar_token(&query.token, &state).await {
        Ok(usuario) => {
            ws.on_upgrade(move |socket| {
                // Instancia o objeto de sessão e inicia o comportamento
                let session = ChatSession::new(socket, state, usuario);
                session.start()
            })
        },
        Err(e) => e.into_response(),
    }
}

// --- Endpoints de Histórico (Delegando para o Usecase no AppState) ---

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
    let mensagens = state
        .chat_usecase
        .listar_historico_loja_usuario(loja_uuid, usuario_uuid)
        .await?;
    Ok(Json(mensagens))
}

pub async fn marcar_lida(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(mensagem_uuid): axum::extract::Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.chat_usecase.marcar_lida(mensagem_uuid).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

