use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq)]
pub struct AvaliacaoDeLoja {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub nota: Decimal,
    pub comentario: Option<String>,
    pub criado_em: DateTime<Utc>,
}

impl AvaliacaoDeLoja {
    pub fn new(loja_uuid: Uuid, usuario_uuid: Uuid, nota: Decimal, comentario: Option<String>) -> Self {
        Self { uuid: Uuid::new_v4(), loja_uuid, usuario_uuid, nota, comentario, criado_em: Utc::now() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AvaliacaoDeProduto {
    pub uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub produto_uuid: Uuid,
    pub nota: Decimal,
    pub descricao: String,
    pub comentario: Option<String>,
    pub criado_em: DateTime<Utc>,
}

impl AvaliacaoDeProduto {
    pub fn new(usuario_uuid: Uuid, loja_uuid: Uuid, produto_uuid: Uuid, nota: Decimal, descricao: String, comentario: Option<String>) -> Self {
        Self { uuid: Uuid::new_v4(), usuario_uuid, loja_uuid, produto_uuid, nota, descricao, comentario, criado_em: Utc::now() }
    }
}
