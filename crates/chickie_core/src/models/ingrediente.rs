use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::Utc;
use rust_decimal::Decimal;
use utoipa::ToSchema;
use crate::{models::Model, ports::to_proto::ToProto};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Ingrediente {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub unidade_medida: Option<String>,
    pub quantidade: Decimal,
    pub preco_unitario: Decimal,
    pub criado_em: chrono::DateTime<chrono::Utc>,
    pub atualizado_em: chrono::DateTime<chrono::Utc>,
}

impl Ingrediente {
    pub fn new(
        nome: String,
        loja_uuid: Uuid,
        unidade_medida: Option<String>,
        preco_unitario: Decimal,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            nome,
            unidade_medida,
            quantidade: Decimal::ZERO,
            preco_unitario,
            criado_em: Utc::now(),
            atualizado_em: Utc::now(),
        }
    }
}

impl ToProto<crate::proto::Ingrediente> for Ingrediente {
    fn to_proto(&self) -> crate::proto::Ingrediente {
        crate::proto::Ingrediente {
            uuid: self.uuid.to_string(),
            loja_uuid: self.loja_uuid.to_string(),
            nome: self.nome.clone(),
            unidade_medida: self.unidade_medida.clone().unwrap_or_default(),
            quantidade: self.quantidade.to_string(),
            preco_unitario: self.preco_unitario.to_string(),
            criado_em: self.criado_em.to_rfc3339(),
            atualizado_em: self.atualizado_em.to_rfc3339(),
        }
    }
}


impl Model for Ingrediente {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
