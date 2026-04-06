use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "enderecos_loja")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub loja_uuid: Uuid,
    #[sea_orm(nullable)]
    pub cep: Option<String>,
    pub logradouro: String,
    pub numero: String,
    #[sea_orm(nullable)]
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    #[sea_orm(nullable)]
    pub latitude: Option<Decimal>,
    #[sea_orm(nullable)]
    pub longitude: Option<Decimal>,
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
