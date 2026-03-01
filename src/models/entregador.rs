use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::utils::agora;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Entregador {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub telefone: Option<String>,
    pub veiculo: Option<String>,
    pub placa: Option<String>,
    pub disponivel: bool,
    pub criado_em: String,
}

impl Entregador {
    pub fn new(
        nome: String,
        loja_uuid: Uuid,
        telefone: Option<String>,
        veiculo: Option<String>,
        placa: Option<String>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            nome,
            telefone,
            veiculo,
            placa,
            disponivel: false,
            criado_em: agora()
        }
    }
}

#[derive(Debug)]
pub struct Entregadores {
    pub payload: Vec<Entregador>,
    pub limit: i32,
    pub offset: i32,
}

impl Entregadores {
    pub fn new(payload: Vec<Entregador>, limit: i32, offset: i32) -> Self {
        Self { payload, limit, offset }
    }
}