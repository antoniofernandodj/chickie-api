use async_trait::async_trait;
use redis::{AsyncCommands, Client};
use crate::models::MensagemChat;
use crate::ports::ChatPublisherPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct RedisChatAdapter {
    client: Client,
}

impl RedisChatAdapter {
    pub fn new(redis_url: &str) -> Result<Self, String> {
        let client = Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self { client })
    }
}

#[async_trait]
impl ChatPublisherPort for RedisChatAdapter {
    async fn publicar_mensagem(&self, msg: MensagemChat) -> DomainResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        
        // Publicamos em dois canais: um para a loja e outro para o usuário
        let msg_json = serde_json::to_string(&msg)
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        
        let canal_loja = format!("chat:loja:{}", msg.loja_uuid);
        let canal_usuario = format!("chat:usuario:{}", msg.usuario_uuid);
        
        let _: () = conn.publish(canal_loja, &msg_json)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let _: () = conn.publish(canal_usuario, &msg_json)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        
        Ok(())
    }
}
