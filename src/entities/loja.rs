use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Utc, NaiveTime};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "lojas")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub uuid: Uuid,
    pub nome: String,
    pub slug: String,
    #[sea_orm(nullable)]
    pub descricao: Option<String>,
    pub email: String,
    #[sea_orm(nullable)]
    pub telefone: Option<String>,
    pub ativa: bool,
    #[sea_orm(nullable)]
    pub logo_url: Option<String>,
    #[sea_orm(nullable)]
    pub banner_url: Option<String>,
    #[sea_orm(nullable)]
    pub horario_abertura: Option<NaiveTime>,
    #[sea_orm(nullable)]
    pub horario_fechamento: Option<NaiveTime>,
    #[sea_orm(column_type = "Integer", array, nullable)]
    pub dias_funcionamento: Option<Vec<i32>>,
    #[sea_orm(nullable)]
    pub tempo_preparo_min: Option<i32>,
    #[sea_orm(nullable)]
    pub taxa_entrega: Option<Decimal>,
    #[sea_orm(nullable)]
    pub valor_minimo_pedido: Option<Decimal>,
    #[sea_orm(nullable)]
    pub raio_entrega_km: Option<Decimal>,
    #[sea_orm(nullable, column_type = "Uuid")]
    pub criado_por: Option<Uuid>,
    pub criado_em: ::chrono::DateTime<Utc>,
    pub atualizado_em: ::chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::usuario::Entity",
        from = "Column::CriadoPor",
        to = "super::usuario::Column::Uuid"
    )]
    Usuario,
}

impl Related<super::usuario::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Usuario.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
