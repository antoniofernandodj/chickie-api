use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "uso_cupons")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub cupom_uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub usuario_uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub pedido_uuid: Uuid,
    pub valor_desconto: Decimal,
    pub usado_em: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::cupom::Entity",
        from = "Column::CupomUuid",
        to = "super::cupom::Column::Uuid"
    )]
    Cupom,
    #[sea_orm(
        belongs_to = "super::usuario::Entity",
        from = "Column::UsuarioUuid",
        to = "super::usuario::Column::Uuid"
    )]
    Usuario,
    #[sea_orm(
        belongs_to = "super::pedido::Entity",
        from = "Column::PedidoUuid",
        to = "super::pedido::Column::Uuid"
    )]
    Pedido,
}

impl Related<super::cupom::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cupom.def()
    }
}

impl Related<super::usuario::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Usuario.def()
    }
}

impl Related<super::pedido::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pedido.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
