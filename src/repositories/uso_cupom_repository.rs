use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::uso_cupom::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct UsoCupomRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl UsoCupomRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Model>, String> {
        uso_cupom::Entity::find()
            .filter(uso_cupom::Column::UsuarioUuid.eq(SeaUuid::from(usuario_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_cupom(&self, cupom_uuid: Uuid) -> Result<Vec<Model>, String> {
        uso_cupom::Entity::find()
            .filter(uso_cupom::Column::CupomUuid.eq(SeaUuid::from(cupom_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn contar_usos_usuario(&self, usuario_uuid: Uuid, cupom_uuid: Uuid) -> Result<u32, String> {
        let usos = self.buscar_por_usuario(usuario_uuid).await?;
        let count = usos.into_iter()
            .filter(|u| u.cupom_uuid == SeaUuid::from(cupom_uuid))
            .count();
        Ok(count as u32)
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for UsoCupomRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        uso_cupom::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Uso de cupom" 
    }



    async fn listar_todos_por_loja(&self, _: Uuid) -> Result<Vec<Model>, String> {
        Err("nao se aplica - uso de cupom nao esta vinculado diretamente a lojas".into())
    }
}
