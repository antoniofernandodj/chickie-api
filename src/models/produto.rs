use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use utoipa::ToSchema;

use crate::models::Model;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Produto {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub categoria_uuid: Uuid,
    pub nome: String,
    pub descricao: Option<String>,
    pub preco: Decimal,
    pub imagem_url: Option<String>,
    pub disponivel: bool,
    pub tempo_preparo_min: Option<i32>,
    pub destaque: bool,
    pub criado_em: DateTime<Utc>,
    pub atualizado_em: DateTime<Utc>,
}

impl Produto {
    pub fn new(
        nome: String,
        descricao: Option<String>,
        preco: Decimal,
        categoria_uuid: Uuid,
        loja_uuid: Uuid,
        tempo_preparo_min: Option<i32>,
    ) -> Self {

        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            categoria_uuid,
            nome,
            descricao,
            preco,
            imagem_url: None,
            disponivel: false,
            tempo_preparo_min,
            destaque: false,
            criado_em: Utc::now(),
            atualizado_em: Utc::now(),
        }
    }
}

impl Model for Produto {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
