use uuid::Uuid;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::{Utc, NaiveDate};
use rust_decimal::Decimal;
use utoipa::ToSchema;
use crate::{models::Model, ports::to_proto::ToProto};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Funcionario {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub cargo: Option<String>,
    pub salario: Option<Decimal>,
    pub data_admissao: NaiveDate,
    pub criado_em: chrono::DateTime<chrono::Utc>,
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
            criado_em: Utc::now(),
        }
    }
}

impl ToProto<crate::proto::Funcionario> for Funcionario {
    fn to_proto(&self) -> crate::proto::Funcionario {
        crate::proto::Funcionario {
            uuid: self.uuid.to_string(),
            loja_uuid: self.loja_uuid.to_string(),
            usuario_uuid: self.usuario_uuid.to_string(),
            cargo: self.cargo.clone().unwrap_or_default(),
            salario: self.salario.map(|s| s.to_string()).unwrap_or_default(),
            data_admissao: self.data_admissao.to_string(),
        }
    }
}

impl Model for Funcionario {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
