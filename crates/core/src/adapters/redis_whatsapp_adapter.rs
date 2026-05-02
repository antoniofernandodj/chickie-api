use async_trait::async_trait;
use redis::{AsyncCommands, Client};
use crate::ports::{WhatsAppConversationPort, WhatsAppConversationState};
use crate::domain::errors::{DomainError, DomainResult};

pub struct RedisWhatsAppConversationAdapter {
    client: Client,
}

impl RedisWhatsAppConversationAdapter {
    pub fn new(redis_url: &str) -> Result<Self, String> {
        let client = Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self { client })
    }
}

#[async_trait]
impl WhatsAppConversationPort for RedisWhatsAppConversationAdapter {
    async fn get_state(&self, phone_number: &str) -> DomainResult<Option<WhatsAppConversationState>> {
        let mut conn = self.client.get_multiplexed_async_connection()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        
        let key = format!("whatsapp:conversation:{}", phone_number);
        let val: Option<String> = conn.get(key)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        
        match val {
            Some(s) => {
                let state: WhatsAppConversationState = serde_json::from_str(&s)
                    .map_err(|e| DomainError::Internal(e.to_string()))?;
                Ok(Some(state))
            },
            None => Ok(None)
        }
    }

    async fn set_state(&self, phone_number: &str, state: WhatsAppConversationState, ttl_seconds: u64) -> DomainResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        
        let key = format!("whatsapp:conversation:{}", phone_number);
        let val = serde_json::to_string(&state)
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        
        let _: () = conn.set_ex(key, val, ttl_seconds)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        
        Ok(())
    }

    async fn delete_state(&self, phone_number: &str) -> DomainResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        
        let key = format!("whatsapp:conversation:{}", phone_number);
        let _: () = conn.del(key)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        
        Ok(())
    }
}
