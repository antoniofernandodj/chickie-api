use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait,QueryOrder};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::cliente::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct ClienteRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl ClienteRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Model>, String> {
        cliente::Entity::find()
            .filter(cliente::Column::UsuarioUuid.eq(SeaUuid::from(usuario_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        cliente::Entity::find()
            .filter(cliente::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for ClienteRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        cliente::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Cliente" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
