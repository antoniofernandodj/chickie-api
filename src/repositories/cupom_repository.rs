use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait,QueryOrder};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::cupom::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct CupomRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl CupomRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_codigo(&self, codigo: &str, loja_uuid: Uuid) -> Result<Option<Model>, String> {
        let todos = self.buscar_por_loja(loja_uuid).await?;
        let codigo_upper = codigo.to_uppercase();
        let encontrado = todos.into_iter()
            .find(|c| c.codigo.to_uppercase() == codigo_upper);
        Ok(encontrado)
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        cupom::Entity::find()
            .filter(cupom::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_ativos(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        cupom::Entity::find()
            .filter(cupom::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .filter(cupom::Column::Status.eq("Ativo"))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for CupomRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        cupom::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Cupom" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
