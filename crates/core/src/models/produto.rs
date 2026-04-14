use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::Utc;
use rust_decimal::Decimal;
use utoipa::ToSchema;

use crate::models::Model;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
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
    pub criado_em: chrono::DateTime<chrono::Utc>,
    pub atualizado_em: chrono::DateTime<chrono::Utc>,
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

impl Produto {
    pub fn to_proto(&self) -> crate::proto::Produto {
        crate::proto::Produto {
            uuid: self.uuid.to_string(),
            loja_uuid: self.loja_uuid.to_string(),
            categoria_uuid: self.categoria_uuid.to_string(),
            nome: self.nome.clone(),
            descricao: self.descricao.clone().unwrap_or_default(),
            preco: self.preco.to_string(),
            imagem_url: self.imagem_url.clone().unwrap_or_default(),
            disponivel: self.disponivel,
            tempo_preparo_min: self.tempo_preparo_min.unwrap_or_default(),
            destaque: self.destaque,
            criado_em: self.criado_em.to_rfc3339(),
            atualizado_em: self.atualizado_em.to_rfc3339(),
        }
    }
}
