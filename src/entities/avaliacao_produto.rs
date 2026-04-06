use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "avaliacoes_produto")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub usuario_uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub loja_uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub produto_uuid: Uuid,
    pub nota: Decimal,
    pub descricao: String,
    #[sea_orm(nullable)]
    pub comentario: Option<String>,
    pub criado_em: ::chrono::DateTime<Utc>,
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
    #[sea_orm(
        belongs_to = "super::produto::Entity",
        from = "Column::ProdutoUuid",
        to = "super::produto::Column::Uuid"
    )]
    Produto,
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

impl Related<super::produto::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Produto.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
