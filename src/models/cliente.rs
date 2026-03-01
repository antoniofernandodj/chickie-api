use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

use crate::utils::agora;


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Cliente {
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub uuid: Uuid,
    pub criado_em: String,
}

impl Cliente {
    pub fn new(
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
    ) -> Self {
        Self {
            usuario_uuid,
            loja_uuid,
            uuid: Uuid::new_v4(),
            criado_em: agora()
        }
    }
}
