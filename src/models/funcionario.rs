use uuid::Uuid;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::Utc;

use crate::models::Model;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Funcionario {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub cargo: Option<String>,
    pub salario: Option<f64>,
    pub data_admissao: String,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl Funcionario {
    pub fn new(
        loja_uuid: Uuid,
        usuario_uuid: Uuid,
        cargo: Option<String>,
        salario: Option<f64>,
        data_admissao: String,
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
