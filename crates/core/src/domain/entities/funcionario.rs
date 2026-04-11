use uuid::Uuid;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq)]
pub struct Funcionario {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub cargo: Option<String>,
    pub salario: Option<Decimal>,
    pub data_admissao: NaiveDate,
    pub criado_em: DateTime<Utc>,
}

impl Funcionario {
    pub fn new(loja_uuid: Uuid, usuario_uuid: Uuid, cargo: Option<String>, salario: Option<Decimal>, data_admissao: NaiveDate) -> Self {
        Self { uuid: Uuid::new_v4(), loja_uuid, usuario_uuid, cargo, salario, data_admissao, criado_em: Utc::now() }
    }
}
