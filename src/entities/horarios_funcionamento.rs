use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Utc, NaiveTime};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "horarios_funcionamento")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub loja_uuid: Uuid,
    pub dia_semana: i32,
    pub abertura: NaiveTime,
    pub fechamento: NaiveTime,
    pub ativo: bool,
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
