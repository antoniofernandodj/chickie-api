use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;
use utoipa::ToSchema;

use crate::models::Model;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
    pub fn new(
        loja_uuid: Uuid,
        usuario_uuid: Uuid,
        cargo: Option<String>,
        salario: Option<Decimal>,
        data_admissao: NaiveDate,
    ) -> Self {

        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            usuario_uuid,
            cargo,
            salario,
            data_admissao,
            criado_em: Utc::now()
        }

    }
}


impl Model for Funcionario {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
