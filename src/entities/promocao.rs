use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "promocoes")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub loja_uuid: Uuid,
    pub nome: String,
    pub descricao: String,
    pub tipo_desconto: String,
    #[sea_orm(nullable)]
    pub valor_desconto: Option<Decimal>,
    #[sea_orm(nullable)]
    pub valor_minimo: Option<Decimal>,
    pub data_inicio: String,
    pub data_fim: String,
    #[sea_orm(column_type = "Integer", array, nullable)]
    pub dias_semana_validos: Option<Vec<i32>>,
    pub tipo_escopo: String,
    #[sea_orm(nullable, column_type = "Uuid")]
    pub produto_uuid: Option<Uuid>,
    #[sea_orm(nullable, column_type = "Uuid")]
    pub categoria_uuid: Option<Uuid>,
    pub status: String,
    pub prioridade: i32,
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
    #[sea_orm(
        belongs_to = "super::produto::Entity",
        from = "Column::ProdutoUuid",
        to = "super::produto::Column::Uuid"
    )]
    Produto,
    #[sea_orm(
        belongs_to = "super::categoria_produtos::Entity",
        from = "Column::CategoriaUuid",
        to = "super::categoria_produtos::Column::Uuid"
    )]
    CategoriaProdutos,
}

impl Related<super::loja::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Loja.def()
    }
}

impl Related<super::produto::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Produto.def()
    }
}

impl Related<super::categoria_produtos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CategoriaProdutos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// Domain logic for promocoes
#[allow(dead_code)]
impl Model {
    pub fn calcular_desconto(&self, subtotal: Decimal) -> Decimal {
        if self.status != "Ativo" {
            return Decimal::from(0);
        }

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
            _ => Decimal::from(0),
        }
    }

    pub fn eh_aplicavel(&self, subtotal: Decimal, dia_semana: Option<i32>) -> bool {
        if self.status != "Ativo" {
            return false;
        }

        // Check minimum value
        if let Some(valor_minimo) = &self.valor_minimo {
            if subtotal < *valor_minimo {
                return false;
            }
        }

        // Check day of week
        if let Some(dia) = dia_semana {
            if let Some(dias_validos) = &self.dias_semana_validos {
                if !dias_validos.contains(&dia) {
                    return false;
                }
            }
        }

        true
    }
}
