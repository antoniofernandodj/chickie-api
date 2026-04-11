use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq)]
pub struct Ingrediente {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub unidade_medida: Option<String>,
    pub quantidade: Decimal,
    pub preco_unitario: Decimal,
    pub criado_em: DateTime<Utc>,
    pub atualizado_em: DateTime<Utc>,
}

impl Ingrediente {
    pub fn new(nome: String, loja_uuid: Uuid, unidade_medida: Option<String>, preco_unitario: Decimal) -> Self {
        let now = Utc::now();
        Self { uuid: Uuid::new_v4(), loja_uuid, nome, unidade_medida, quantidade: Decimal::ZERO, preco_unitario, criado_em: now, atualizado_em: now }
    }
}
