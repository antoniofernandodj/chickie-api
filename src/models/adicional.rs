use rust_decimal::Decimal;
use uuid::Uuid;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use crate::models::Model;


#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Adicional {
    pub uuid: Uuid,
    pub nome: String,
    pub loja_uuid: Uuid,
    pub disponivel: bool,
    pub descricao: String,
    pub preco: Decimal,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl Adicional {
    pub fn new(
        nome: String,
        loja_uuid: Uuid,
        descricao: String,
        preco: Decimal,
    ) -> Self {

        Self {
            nome,
            loja_uuid,
            disponivel: false,
            descricao,
            preco,
            uuid: Uuid::new_v4(),
            criado_em: Utc::now()
        }

    }
}

impl Model for Adicional {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
