use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::Utc;

use crate::models::Model;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Produto {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub categoria_uuid: Uuid,
    pub nome: String,
    pub descricao: Option<String>,
    pub preco: f64,
    pub imagem_url: Option<String>,
    pub disponivel: bool,
    pub tempo_preparo_min: Option<i32>,
    pub destaque: bool,
    pub criado_em: chrono::DateTime<chrono::Utc>,
    pub atualizado_em: chrono::DateTime<chrono::Utc>,
}

impl Produto {
    pub fn new(
        nome: String,
        descricao: Option<String>,
        preco: f64,
        categoria_uuid: Uuid,
        loja_uuid: Uuid,
        tempo_preparo_min: Option<i32>,
    ) -> Self {

        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            categoria_uuid,
            nome,
            descricao,
            preco,
            imagem_url: None,
            disponivel: false,
            tempo_preparo_min,
            destaque: false,
            criado_em: Utc::now(),
            atualizado_em: Utc::now(),
        }
    }
}

impl Model for Produto {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
