use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::errors::DomainResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WhatsAppIdentityType {
    Authenticated,
    Guest,
    Anonymous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppConversationState {
    pub phone_number: String,
    pub identity_type: WhatsAppIdentityType,
    pub identifier: Option<Uuid>, // user_id or guest_token (which is codigo in our case? or just use a token)
    pub current_step: String,
    pub context: serde_json::Value,
    pub last_interaction: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait WhatsAppConversationPort: Send + Sync {
    async fn get_state(&self, phone_number: &str) -> DomainResult<Option<WhatsAppConversationState>>;
    async fn set_state(&self, phone_number: &str, state: WhatsAppConversationState, ttl_seconds: u64) -> DomainResult<()>;
    async fn delete_state(&self, phone_number: &str) -> DomainResult<()>;
}
