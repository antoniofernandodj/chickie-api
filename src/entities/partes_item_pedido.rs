use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "partes_item_pedido")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub loja_uuid: Uuid,
    #[sea_orm(nullable, column_type = "Uuid")]
    pub item_uuid: Option<Uuid>,
    pub produto_nome: String,
    #[sea_orm(column_type = "Uuid")]
    pub produto_uuid: Uuid,
    pub preco_unitario: Decimal,
    pub posicao: i32,
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
        belongs_to = "super::item_pedido::Entity",
        from = "Column::ItemUuid",
        to = "super::item_pedido::Column::Uuid"
    )]
    ItemPedido,
}

impl Related<super::loja::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Loja.def()
    }
}

impl Related<super::item_pedido::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemPedido.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
