use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::Utc;
use utoipa::ToSchema;
use crate::{models::Model, ports::to_proto::ToProto};

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
            criado_em: Utc::now(),
        }
    }
}

impl ToProto<crate::proto::Entregador> for Entregador {
    fn to_proto(&self) -> crate::proto::Entregador {
        crate::proto::Entregador {
            uuid: self.uuid.to_string(),
            loja_uuid: self.loja_uuid.to_string(),
            usuario_uuid: self.usuario_uuid.to_string(),
            veiculo: self.veiculo.clone().unwrap_or_default(),
            placa: self.placa.clone().unwrap_or_default(),
            disponivel: self.disponivel,
        }
    }
}

impl Model for Entregador {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
