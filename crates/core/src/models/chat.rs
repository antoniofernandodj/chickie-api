use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use crate::models::Model;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MensagemChat {
    pub uuid: Uuid,
    pub pedido_uuid: Option<Uuid>,
    pub loja_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub remetente_uuid: Uuid,
    pub texto: String,
    pub lida: bool,
    pub criado_em: DateTime<Utc>,
}

impl Model for MensagemChat {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMensagemRequest {
    pub pedido_uuid: Option<Uuid>,
    pub loja_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub texto: String,
}
