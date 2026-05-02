use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use crate::models::Model;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WhatsAppBinding {
    pub uuid: Uuid,
    pub user_id: Uuid,
    pub phone_number: String,
    pub verified: bool,
    pub verification_code_hash: Option<String>,
    pub verification_expires_at: Option<DateTime<Utc>>,
    pub criado_em: DateTime<Utc>,
    pub atualizado_em: DateTime<Utc>,
}

impl WhatsAppBinding {
    pub fn new(user_id: Uuid, phone_number: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            user_id,
            phone_number,
            verified: false,
            verification_code_hash: None,
            verification_expires_at: None,
            criado_em: Utc::now(),
            atualizado_em: Utc::now(),
        }
    }
}

impl Model for WhatsAppBinding {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}
