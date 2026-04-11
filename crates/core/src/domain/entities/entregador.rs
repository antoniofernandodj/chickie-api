use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct Entregador {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub veiculo: Option<String>,
    pub placa: Option<String>,
    pub disponivel: bool,
    pub criado_em: DateTime<Utc>,
}

impl Entregador {
    pub fn new(loja_uuid: Uuid, usuario_uuid: Uuid, veiculo: Option<String>, placa: Option<String>) -> Self {
        Self { uuid: Uuid::new_v4(), loja_uuid, usuario_uuid, veiculo, placa, disponivel: false, criado_em: Utc::now() }
    }
}
