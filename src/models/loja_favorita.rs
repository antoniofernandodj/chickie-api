use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::Utc;

use crate::models::Model;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LojaFavorita {
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub uuid: Uuid,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl LojaFavorita {
    pub fn new(usuario_uuid: Uuid, loja_uuid: Uuid) -> Self {
        Self {
            usuario_uuid,
            loja_uuid,
            uuid: Uuid::new_v4(),
            criado_em: Utc::now()
        }
    }
}

impl Model for LojaFavorita {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
