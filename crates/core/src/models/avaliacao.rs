use uuid::Uuid;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use rust_decimal::Decimal;
use utoipa::ToSchema;

use crate::models::Model;


#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct AvaliacaoDeLoja {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub nota: Decimal,
    pub comentario: Option<String>,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

/// Avaliação de loja com dados do usuário (nome e email) obtidos via JOIN.
/// Usada no endpoint de listagem para evitar queries N+1.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct AvaliacaoDeLojaComUsuario {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub usuario_nome: String,
    pub usuario_email: String,
    pub nota: Decimal,
    pub comentario: Option<String>,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl AvaliacaoDeLoja {
    pub fn new(
        loja_uuid: Uuid,
        usuario_uuid: Uuid,
        nota: Decimal,
        comentario: Option<String>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            usuario_uuid,
            nota,
            comentario,
            criado_em: Utc::now()
        }
    }

    pub fn to_proto(&self) -> crate::proto::AvaliacaoLoja {
        crate::proto::AvaliacaoLoja {
            uuid: self.uuid.to_string(),
            loja_uuid: self.loja_uuid.to_string(),
            usuario_uuid: self.usuario_uuid.to_string(),
            nota: self.nota.to_string(),
            comentario: self.comentario.clone().unwrap_or_default(),
            criado_em: self.criado_em.to_rfc3339(),
            usuario_nome: "".to_string(),
            usuario_email: "".to_string(),
        }
    }
}

impl AvaliacaoDeLojaComUsuario {
    pub fn to_proto(&self) -> crate::proto::AvaliacaoLoja {
        crate::proto::AvaliacaoLoja {
            uuid: self.uuid.to_string(),
            loja_uuid: self.loja_uuid.to_string(),
            usuario_uuid: self.usuario_uuid.to_string(),
            nota: self.nota.to_string(),
            comentario: self.comentario.clone().unwrap_or_default(),
            criado_em: self.criado_em.to_rfc3339(),
            usuario_nome: self.usuario_nome.clone(),
            usuario_email: self.usuario_email.clone(),
        }
    }
}


#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct AvaliacaoDeProduto {
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub produto_uuid: Uuid,
    pub comentario: Option<String>,
    pub nota: Decimal,
    pub descricao: String,
    pub uuid: Uuid,
    pub criado_em: chrono::DateTime<chrono::Utc>
}

impl AvaliacaoDeProduto {
    pub fn new(
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
        produto_uuid: Uuid,
        comentario: Option<String>,
        nota: Decimal,
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
            criado_em: Utc::now()
        }
    }

    pub fn to_proto(&self) -> crate::proto::AvaliacaoProduto {
        crate::proto::AvaliacaoProduto {
            uuid: self.uuid.to_string(),
            usuario_uuid: self.usuario_uuid.to_string(),
            loja_uuid: self.loja_uuid.to_string(),
            produto_uuid: self.produto_uuid.to_string(),
            nota: self.nota.to_string(),
            descricao: self.descricao.clone(),
            comentario: self.comentario.clone().unwrap_or_default(),
            criado_em: self.criado_em.to_rfc3339(),
        }
    }
}

impl Model for AvaliacaoDeProduto {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}

impl Model for AvaliacaoDeLoja {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}