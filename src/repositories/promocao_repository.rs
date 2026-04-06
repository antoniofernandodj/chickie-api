use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait,QueryOrder};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::promocao::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct PromocaoRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl PromocaoRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        promocao::Entity::find()
            .filter(promocao::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_ativas(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        promocao::Entity::find()
            .filter(promocao::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .filter(promocao::Column::Status.eq("Ativo"))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_prioridade(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        promocao::Entity::find()
            .filter(promocao::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .filter(promocao::Column::Status.eq("Ativo"))
            .order_by_desc(promocao::Column::Prioridade)
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for PromocaoRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        promocao::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Promoção" 
    }
    
    fn entity_gender_suffix(&self) -> &'static str { 
        "o" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
