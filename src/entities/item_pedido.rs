use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "itens_pedido")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub loja_uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub pedido_uuid: Uuid,
    pub quantidade: i32,
    #[sea_orm(nullable)]
    pub observacoes: Option<String>,
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
        belongs_to = "super::pedido::Entity",
        from = "Column::PedidoUuid",
        to = "super::pedido::Column::Uuid"
    )]
    Pedido,
}

impl Related<super::loja::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Loja.def()
    }
}

impl Related<super::pedido::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pedido.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
