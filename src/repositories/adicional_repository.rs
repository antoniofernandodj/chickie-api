use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait,ActiveModelTrait,Set};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::adicional::{self, Entity, Model, ActiveModel},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct AdicionalRepository { 
    db: Arc<DatabaseConnection> 
}

impl AdicionalRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        adicional::Entity::find()
            .filter(adicional::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        adicional::Entity::find()
            .filter(adicional::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .filter(adicional::Column::Disponivel.eq(true))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn marcar_indisponivel(&self, uuid: Uuid) -> Result<(), String> {
        let model = self.buscar_por_uuid(uuid).await
            .map_err(|e| e.to_string())?
            .ok_or("Adicional não encontrado".to_string())?;
        
        let mut active: ActiveModel = model.into();
        active.disponivel = Set(false);
        
        active.update(&*self.db)
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for AdicionalRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        adicional::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Adicional" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
