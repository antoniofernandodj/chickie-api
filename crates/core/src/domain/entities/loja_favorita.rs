use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct LojaFavorita {
    pub uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub criado_em: DateTime<Utc>,
}

impl LojaFavorita {
    pub fn new(usuario_uuid: Uuid, loja_uuid: Uuid) -> Self {
        Self { uuid: Uuid::new_v4(), usuario_uuid, loja_uuid, criado_em: Utc::now() }
    }
}
