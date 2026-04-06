use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use utoipa::ToSchema;

use crate::models::Model;


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AvaliacaoDeLoja {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub nota: Decimal,
    pub comentario: Option<String>,
    pub criado_em: DateTime<Utc>,
}

impl AvaliacaoDeLoja {
    pub fn new(
        loja_uuid: Uuid,
        usuario_uuid: Uuid,
        nota: Decimal,
        comentario: Option<String>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            usuario_uuid,
            nota,
            comentario,
            criado_em: Utc::now()
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AvaliacaoDeProduto {
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub produto_uuid: Uuid,
    pub comentario: Option<String>,
    pub nota: Decimal,
    pub descricao: String,
    pub uuid: Uuid,
    pub criado_em: DateTime<Utc>
}

impl AvaliacaoDeProduto {
    pub fn new(
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
        produto_uuid: Uuid,
        comentario: Option<String>,
        nota: Decimal,
        descricao: String,
    ) -> Self {
        Self {
            usuario_uuid,
            loja_uuid,
            produto_uuid,
            comentario,
            nota,
            descricao,
            uuid: Uuid::new_v4(),
            criado_em: Utc::now()
        }
    }
}

impl Model for AvaliacaoDeProduto {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}

impl Model for AvaliacaoDeLoja {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}