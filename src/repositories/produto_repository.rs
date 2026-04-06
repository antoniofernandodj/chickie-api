use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QueryOrder};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::produto::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct ProdutoRepository {
    db: Arc<DatabaseConnection>
}

#[allow(dead_code)]
impl ProdutoRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        produto::Entity::find()
            .filter(produto::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_categoria(&self, categoria_uuid: Uuid) -> Result<Vec<Model>, String> {
        produto::Entity::find()
            .filter(produto::Column::CategoriaUuid.eq(SeaUuid::from(categoria_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        produto::Entity::find()
            .filter(produto::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .filter(produto::Column::Disponivel.eq(true))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_nome(&self, nome: &str, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        // Filter in memory as sea-orm doesn't have native LIKE support
        let all_produtos = self.buscar_por_loja(loja_uuid).await?;
        let nome_lower = nome.to_lowercase();
        let filtrados = all_produtos.into_iter()
            .filter(|p| p.nome.to_lowercase().contains(&nome_lower))
            .collect();
        Ok(filtrados)
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for ProdutoRepository {
    fn db(&self) -> &DatabaseConnection {
        &*self.db
    }

    fn entity(&self) -> Entity {
        produto::Entity
    }

    fn entity_name(&self) -> &'static str {
        "Produto"
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
