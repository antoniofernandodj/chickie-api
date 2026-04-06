use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::usuario::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct UsuarioRepository {
    db: Arc<DatabaseConnection>
}

#[allow(dead_code)]
impl UsuarioRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn buscar_por_email(&self, email: &str) -> Result<Option<Model>, String> {
        usuario::Entity::find()
            .filter(usuario::Column::Email.eq(email))
            .one(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_username(&self, username: &str) -> Result<Option<Model>, String> {
        usuario::Entity::find()
            .filter(usuario::Column::Username.eq(username))
            .one(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_telefone(&self, telefone: &str) -> Result<Option<Model>, String> {
        usuario::Entity::find()
            .filter(usuario::Column::Telefone.eq(telefone))
            .one(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for UsuarioRepository {
    fn db(&self) -> &DatabaseConnection {
        &*self.db
    }

    fn entity(&self) -> Entity {
        usuario::Entity
    }

    fn entity_name(&self) -> &'static str {
        "Usuário"
    }

    async fn listar_todos_por_loja(&self, _: Uuid) -> Result<Vec<Model>, String> {
        Err("não se aplica".into())
    }
}
