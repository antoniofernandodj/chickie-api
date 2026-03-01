use uuid::Uuid;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

use crate::utils::agora;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Funcionario {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub email: Option<String>,
    pub cargo: Option<String>,
    pub salario: Option<f64>,
    pub data_admissao: String,
    pub criado_em: String,
}

impl Funcionario {
    pub fn new(
        loja_uuid: Uuid,
        nome: String,
        email: Option<String>,
        cargo: Option<String>,
        salario: Option<f64>,
        data_admissao: String,
    ) -> Self {

        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            nome,
            email,
            cargo,
            salario,
            data_admissao,
            criado_em: agora()
        }

    }
}
