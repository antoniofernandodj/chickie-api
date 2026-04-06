use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::Utc;
use utoipa::ToSchema;
use crate::models::Model;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Entregador {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub veiculo: Option<String>,
    pub placa: Option<String>,
    pub disponivel: bool,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl Entregador {
    pub fn new(
        loja_uuid: Uuid,
        usuario_uuid: Uuid,
        veiculo: Option<String>,
        placa: Option<String>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            usuario_uuid,
            veiculo,
            placa,
            disponivel: false,
            criado_em: Utc::now()
        }
    }
}


impl Model for Entregador {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
