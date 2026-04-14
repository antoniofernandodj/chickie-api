use uuid::Uuid;
use sqlx::FromRow;
use crate::{models::Model, ports::to_proto::ToProto};
use serde::{Serialize, Deserialize};
use chrono::Utc;
use utoipa::ToSchema;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct CategoriaProdutos {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub descricao: Option<String>,
    pub ordem: Option<i32>,
    pub pizza_mode: bool,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl CategoriaProdutos {
    pub fn new(
        nome: String,
        descricao: Option<String>,
        loja_uuid: Uuid,
        ordem: Option<i32>,
        pizza_mode: bool,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            nome,
            descricao,
            ordem,
            pizza_mode,
            criado_em: Utc::now()
        }
    }
}

impl Model for CategoriaProdutos {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}


impl ToProto<crate::proto::Categoria> for CategoriaProdutos {
    fn to_proto(&self) -> crate::proto::Categoria {
        
        crate::proto::Categoria {
            uuid: self.uuid.to_string(),
            loja_uuid: self.loja_uuid.to_string(),
            nome: self.nome.clone(),
            descricao: self.descricao.clone().unwrap_or_default(),
            ordem: self.ordem.unwrap_or_default(),
            pizza_mode: self.pizza_mode,
            criado_em: self.criado_em.to_rfc3339(),
        }
    }
}


