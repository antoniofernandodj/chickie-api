use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::Utc;
use utoipa::ToSchema;

use crate::{models::Model, ports::to_proto::ToProto};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct LojaFavorita {
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub uuid: Uuid,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl LojaFavorita {
    pub fn new(usuario_uuid: Uuid, loja_uuid: Uuid) -> Self {
        Self {
            usuario_uuid,
            loja_uuid,
            uuid: Uuid::new_v4(),
            criado_em: Utc::now()
        }
    }

}

impl ToProto<crate::proto::LojaFavorita> for LojaFavorita {
    fn to_proto(&self) -> crate::proto::LojaFavorita {
        crate::proto::LojaFavorita {
            uuid: self.uuid.to_string(),
            usuario_uuid: self.usuario_uuid.to_string(),
            loja_uuid: self.loja_uuid.to_string(),
            criado_em: self.criado_em.to_rfc3339(),
        }
    }
}


impl Model for LojaFavorita {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
