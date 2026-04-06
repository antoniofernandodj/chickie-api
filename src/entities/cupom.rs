use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "cupons")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub loja_uuid: Uuid,
    pub codigo: String,
    pub descricao: String,
    pub tipo_desconto: String,
    #[sea_orm(nullable)]
    pub valor_desconto: Option<Decimal>,
    #[sea_orm(nullable)]
    pub valor_minimo: Option<Decimal>,
    pub data_validade: String,
    #[sea_orm(nullable)]
    pub limite_uso: Option<i32>,
    pub uso_atual: i32,
    pub status: String,
    pub criado_em: ::chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::loja::Entity",
        from = "Column::LojaUuid",
        to = "super::loja::Column::Uuid"
    )]
    Loja,
}

impl Related<super::loja::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Loja.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// Domain logic for cupons
#[allow(dead_code)]
impl Model {
    pub fn calcular_desconto(&self, subtotal: Decimal) -> Decimal {
        // Check if cupom is valid
        if self.status != "Ativo" {
            return Decimal::from(0);
        }

        // Check minimum value
        if let Some(valor_minimo) = &self.valor_minimo {
            if subtotal < *valor_minimo {
                return Decimal::from(0);
            }
        }

        // Calculate discount based on type
        match self.tipo_desconto.as_str() {
            "percentual" => {
                if let Some(valor) = &self.valor_desconto {
                    (subtotal * *valor) / Decimal::from(100)
                } else {
                    Decimal::from(0)
                }
            }
            "valor_fixo" => {
                if let Some(valor) = &self.valor_desconto {
                    if *valor <= subtotal {
                        *valor
                    } else {
                        subtotal
                    }
                } else {
                    Decimal::from(0)
                }
            }
            "frete_gratis" => {
                // Frete grátis would be applied to taxa_entrega
                Decimal::from(0)
            }
            _ => Decimal::from(0),
        }
    }

    pub fn eh_aplicavel(&self, subtotal: Decimal) -> bool {
        if self.status != "Ativo" {
            return false;
        }

        // Check usage limit
        if let Some(limite) = self.limite_uso {
            if self.uso_atual >= limite {
                return false;
            }
        }

        // Check minimum value
        if let Some(valor_minimo) = &self.valor_minimo {
            if subtotal < *valor_minimo {
                return false;
            }
        }

        true
    }
}
