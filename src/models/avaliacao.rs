use uuid::Uuid;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

use crate::utils::agora;


#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AvaliacaoDeLoja {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub nota: f64,
    pub comentario: Option<String>,
    pub criado_em: String,
}

impl AvaliacaoDeLoja {
    pub fn new(
        loja_uuid: Uuid,
        usuario_uuid: Uuid,
        nota: f64,
        comentario: Option<String>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            usuario_uuid,
            nota,
            comentario,
            criado_em: agora()
        }
    }
}


#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AvaliacaoDeProduto {
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub produto_uuid: Uuid,
    pub comentario: Option<String>,
    pub nota: f64,
    pub descricao: String,
    pub uuid: Uuid,
    pub criado_em: String
}

impl AvaliacaoDeProduto {
    pub fn new(
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
        produto_uuid: Uuid,
        comentario: Option<String>,
        nota: f64,
        descricao: String,
    ) -> Self {
        Self {
            usuario_uuid,
            loja_uuid,
            produto_uuid,
            comentario,
            nota,
            descricao,
            uuid: Uuid::new_v4(),
            criado_em: agora()
        }
    }
}

