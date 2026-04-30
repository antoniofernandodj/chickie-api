use uuid::Uuid;
use sqlx::FromRow;
use crate::models::Model;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use utoipa::ToSchema;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct CategoriaProdutos {
    pub uuid: Uuid,
    pub loja_uuid: Option<Uuid>,
    pub nome: String,
    pub descricao: Option<String>,
    pub pizza_mode: bool,
    pub drink_mode: bool,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl CategoriaProdutos {
    pub fn new(
        nome: String,
        descricao: Option<String>,
        loja_uuid: Option<Uuid>,
        pizza_mode: bool,
        drink_mode: bool,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            nome,
            descricao,
            pizza_mode,
            drink_mode,
            criado_em: Utc::now()
        }
    }
}

impl Model for CategoriaProdutos {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}

/// Categoria com ordem no contexto de uma loja específica.
/// Categorias sem ordem definida recebem um valor de fallback calculado por criado_em.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct CategoriaProdutosOrdenada {
    pub uuid: Uuid,
    pub loja_uuid: Option<Uuid>,
    pub nome: String,
    pub descricao: Option<String>,
    pub ordem: i32,
    pub pizza_mode: bool,
    pub drink_mode: bool,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct StatusCategoriaGlobal {
    pub uuid: Uuid,
    pub nome: String,
    pub tem_produto: bool,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct OrdemCategoriaProdutos {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub categoria_uuid: Uuid,
    pub ordem: i32,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}
