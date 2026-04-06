use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::endereco_loja::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct EnderecoLojaRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl EnderecoLojaRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        endereco_loja::Entity::find()
            .filter(endereco_loja::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for EnderecoLojaRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        endereco_loja::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Endereço" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
