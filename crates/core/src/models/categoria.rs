use uuid::Uuid;
use sqlx::FromRow;
use crate::models::Model;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use utoipa::ToSchema;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct CategoriaProdutos {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub descricao: Option<String>,
    pub ordem: i32,
    pub pizza_mode: bool,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl CategoriaProdutos {
    pub fn new(
        nome: String,
        descricao: Option<String>,
        loja_uuid: Uuid,
        ordem: i32,
        pizza_mode: bool,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            nome,
            descricao,
            ordem,
            pizza_mode,
            criado_em: Utc::now()
        }
    }
}

impl Model for CategoriaProdutos {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
