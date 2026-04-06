use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::loja_favorita::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct LojaFavoritaRepository { 
    db: Arc<DatabaseConnection> 
}

impl LojaFavoritaRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Model>, String> {
        loja_favorita::Entity::find()
            .filter(loja_favorita::Column::UsuarioUuid.eq(SeaUuid::from(usuario_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        loja_favorita::Entity::find()
            .filter(loja_favorita::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_usuario_e_loja(&self, usuario_uuid: Uuid, loja_uuid: Uuid) -> Result<Option<Model>, String> {
        loja_favorita::Entity::find()
            .filter(loja_favorita::Column::UsuarioUuid.eq(SeaUuid::from(usuario_uuid)))
            .filter(loja_favorita::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .one(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for LojaFavoritaRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        loja_favorita::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Loja favorita" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
