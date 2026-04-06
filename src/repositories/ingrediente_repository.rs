use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::ingrediente::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;
use rust_decimal::Decimal;

pub struct IngredienteRepository { 
    db: Arc<DatabaseConnection> 
}

impl IngredienteRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        ingrediente::Entity::find()
            .filter(ingrediente::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        ingrediente::Entity::find()
            .filter(ingrediente::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .filter(ingrediente::Column::Quantidade.gt(Decimal::from(0)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for IngredienteRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        ingrediente::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Ingrediente" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
