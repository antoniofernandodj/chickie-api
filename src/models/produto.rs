use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

use crate::utils::agora;

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
    pub criado_em: String,
    pub atualizado_em: String,
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
            criado_em: agora(),
            atualizado_em: agora(),
        }
    }
}