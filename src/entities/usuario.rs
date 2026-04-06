use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "usuarios")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub uuid: Uuid,
    pub nome: String,
    pub username: String,
    pub email: String,
    pub senha_hash: String,
    pub celular: String,
    #[sea_orm(nullable)]
    pub telefone: Option<String>,
    pub classe: String,
    pub ativo: bool,
    pub passou_pelo_primeiro_acesso: bool,
    pub modo_de_cadastro: String,
    pub criado_em: ::chrono::DateTime<Utc>,
    pub atualizado_em: ::chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
