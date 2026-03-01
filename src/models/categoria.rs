use uuid::Uuid;
use sqlx::FromRow;
use crate::{models::Model, utils::agora};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CategoriaProdutos {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub descricao: Option<String>,
    pub ordem: Option<i32>,
    pub criado_em: String,
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
            criado_em: agora()
        }
    }
}

impl Model for CategoriaProdutos {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
