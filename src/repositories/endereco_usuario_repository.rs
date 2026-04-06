use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::endereco_usuario::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct EnderecoUsuarioRepository { 
    db: Arc<DatabaseConnection> 
}

impl EnderecoUsuarioRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// Busca todos os enderecos registrados de um usuario
    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Model>, String> {
        endereco_usuario::Entity::find()
            .filter(endereco_usuario::Column::UsuarioUuid.eq(SeaUuid::from(usuario_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Busca um endereco especifico pelo UUID (helper para validacoes)
    pub async fn buscar_por_uuid_e_usuario(&self, uuid: Uuid, usuario_uuid: Uuid) -> Result<Option<Model>, String> {
        endereco_usuario::Entity::find()
            .filter(endereco_usuario::Column::Uuid.eq(SeaUuid::from(uuid)))
            .filter(endereco_usuario::Column::UsuarioUuid.eq(SeaUuid::from(usuario_uuid)))
            .one(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for EnderecoUsuarioRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        endereco_usuario::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Endereco de usuario" 
    }



    async fn listar_todos_por_loja(&self, _: Uuid) -> Result<Vec<Model>, String> {
        Err("nao se aplica - enderecos de usuario nao estao vinculados a lojas".into())
    }
}
