use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "produtos")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub loja_uuid: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub categoria_uuid: Uuid,
    pub nome: String,
    #[sea_orm(nullable)]
    pub descricao: Option<String>,
    pub preco: Decimal,
    #[sea_orm(nullable)]
    pub imagem_url: Option<String>,
    pub disponivel: bool,
    #[sea_orm(nullable)]
    pub tempo_preparo_min: Option<i32>,
    pub destaque: bool,
    pub criado_em: ::chrono::DateTime<Utc>,
    pub atualizado_em: ::chrono::DateTime<Utc>,
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

impl Related<super::categoria_produtos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CategoriaProdutos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
