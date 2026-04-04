use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::Utc;
use crate::models::Model;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Ingrediente {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub unidade_medida: Option<String>,
    pub quantidade: f64,
    pub preco_unitario: f64,
    pub criado_em: chrono::DateTime<chrono::Utc>,
    pub atualizado_em: chrono::DateTime<chrono::Utc>,
}

impl Ingrediente {
    pub fn new(
        nome: String,
        loja_uuid: Uuid,
        unidade_medida: Option<String>,
        preco_unitario: f64,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            nome,
            unidade_medida,
            quantidade: 0.0,
            preco_unitario,
            criado_em: Utc::now(),
            atualizado_em: Utc::now(),
        }
    }
}

impl Model for Ingrediente {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
