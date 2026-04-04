use uuid::Uuid;
use sqlx::FromRow;
use crate::models::Model;
use serde::{Serialize, Deserialize};
use chrono::Utc;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CategoriaProdutos {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub descricao: Option<String>,
    pub ordem: Option<i32>,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl CategoriaProdutos {
    pub fn new(
        nome: String,
        descricao: Option<String>,
        loja_uuid: Uuid,
        ordem: Option<i32>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            nome,
            descricao,
            ordem,
            criado_em: Utc::now()
        }
    }
}

impl Model for CategoriaProdutos {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
