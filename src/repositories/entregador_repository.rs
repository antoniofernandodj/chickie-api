use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::entregador::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct EntregadorRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl EntregadorRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        entregador::Entity::find()
            .filter(entregador::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        entregador::Entity::find()
            .filter(entregador::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .filter(entregador::Column::Disponivel.eq(true))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Option<Model>, String> {
        entregador::Entity::find()
            .filter(entregador::Column::UsuarioUuid.eq(SeaUuid::from(usuario_uuid)))
            .one(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for EntregadorRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        entregador::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Entregador" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
