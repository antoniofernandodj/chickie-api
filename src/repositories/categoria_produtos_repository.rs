use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait,QueryOrder};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::categoria_produtos::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct CategoriaProdutosRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl CategoriaProdutosRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        categoria_produtos::Entity::find()
            .filter(categoria_produtos::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_nome(&self, nome: &str, loja_uuid: Uuid) -> Result<Option<Model>, String> {
        categoria_produtos::Entity::find()
            .filter(categoria_produtos::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .filter(categoria_produtos::Column::Nome.eq(nome))
            .one(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for CategoriaProdutosRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        categoria_produtos::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Categoria" 
    }
    
    fn entity_gender_suffix(&self) -> &'static str { 
        "a" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
