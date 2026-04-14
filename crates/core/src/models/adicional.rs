use rust_decimal::Decimal;
use uuid::Uuid;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use utoipa::ToSchema;
use crate::models::Model;


#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
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
            disponivel: true,
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

impl Adicional {
    pub fn to_proto(&self) -> crate::proto::Adicional {
        crate::proto::Adicional {
            uuid: self.uuid.to_string(),
            nome: self.nome.clone(),
            loja_uuid: self.loja_uuid.to_string(),
            disponivel: self.disponivel,
            descricao: self.descricao.clone(),
            preco: self.preco.to_string(),
            criado_em: self.criado_em.to_rfc3339(),
        }
    }
}
