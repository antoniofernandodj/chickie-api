use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::funcionario::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct FuncionarioRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl FuncionarioRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        funcionario::Entity::find()
            .filter(funcionario::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_cargo(&self, cargo: &str, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        funcionario::Entity::find()
            .filter(funcionario::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .filter(funcionario::Column::Cargo.eq(cargo))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Option<Model>, String> {
        funcionario::Entity::find()
            .filter(funcionario::Column::UsuarioUuid.eq(SeaUuid::from(usuario_uuid)))
            .one(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for FuncionarioRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        funcionario::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Funcionário" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
