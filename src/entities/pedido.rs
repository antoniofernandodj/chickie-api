use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "pedidos")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub usuario_uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub loja_uuid: Uuid,
    pub status: String,
    pub total: Decimal,
    pub subtotal: Decimal,
    pub taxa_entrega: Decimal,
    #[sea_orm(nullable)]
    pub desconto: Option<Decimal>,
    pub forma_pagamento: String,
    #[sea_orm(nullable)]
    pub observacoes: Option<String>,
    #[sea_orm(nullable)]
    pub tempo_estimado_min: Option<i32>,
    pub criado_em: ::chrono::DateTime<Utc>,
    pub atualizado_em: ::chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::usuario::Entity",
        from = "Column::UsuarioUuid",
        to = "super::usuario::Column::Uuid"
    )]
    Usuario,
    #[sea_orm(
        belongs_to = "super::loja::Entity",
        from = "Column::LojaUuid",
        to = "super::loja::Column::Uuid"
    )]
    Loja,
}

impl Related<super::usuario::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Usuario.def()
    }
}

impl Related<super::loja::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Loja.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// Domain logic for pedido state machine
#[allow(dead_code)]
impl Model {
    pub fn transicoes_permitidas(&self) -> Vec<&str> {
        match self.status.as_str() {
            "criado" => vec!["aguardando_confirmacao_de_loja"],
            "aguardando_confirmacao_de_loja" => vec!["confirmado_pela_loja", "criado"],
            "confirmado_pela_loja" => vec!["em_preparo", "aguardando_confirmacao_de_loja"],
            "em_preparo" => vec!["pronto_para_retirada", "confirmado_pela_loja"],
            "pronto_para_retirada" => vec!["saiu_para_entrega", "em_preparo"],
            "saiu_para_entrega" => vec!["entregue", "pronto_para_retirada"],
            "entregue" => vec![],
            _ => vec![],
        }
    }

    pub fn pode_transicionar_para(&self, novo_status: &str) -> bool {
        self.transicoes_permitidas().contains(&novo_status)
    }

    pub fn is_terminal(&self) -> bool {
        self.status == "entregue"
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum EstadoDePedido {
    Criado,
    AguardandoConfirmacaoDeLoja,
    ConfirmadoPelaLoja,
    EmPreparo,
    ProntoParaRetirada,
    SaiuParaEntrega,
    Entregue,
}

impl EstadoDePedido {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Criado                      => "criado",
            Self::AguardandoConfirmacaoDeLoja => "aguardando_confirmacao_de_loja",
            Self::ConfirmadoPelaLoja          => "confirmado_pela_loja",
            Self::EmPreparo                   => "em_preparo",
            Self::ProntoParaRetirada          => "pronto_para_retirada",
            Self::SaiuParaEntrega             => "saiu_para_entrega",
            Self::Entregue                    => "entregue",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "criado" => Ok(Self::Criado),
            "aguardando_confirmacao_de_loja" => Ok(Self::AguardandoConfirmacaoDeLoja),
            "confirmado_pela_loja" => Ok(Self::ConfirmadoPelaLoja),
            "em_preparo" => Ok(Self::EmPreparo),
            "pronto_para_retirada" => Ok(Self::ProntoParaRetirada),
            "saiu_para_entrega" => Ok(Self::SaiuParaEntrega),
            "entregue" => Ok(Self::Entregue),
            _ => Err(format!("Estado de pedido inválido: {}", s)),
        }
    }
}
