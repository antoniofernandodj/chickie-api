use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait,QueryOrder};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::loja::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;
use chrono::NaiveTime;

pub struct LojaRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl LojaRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_email(&self, email: &str) -> Result<Option<Model>, String> {
        loja::Entity::find()
            .filter(loja::Column::Email.eq(email))
            .one(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_slug(&self, slug: &str) -> Result<Option<Model>, String> {
        loja::Entity::find()
            .filter(loja::Column::Slug.eq(slug))
            .one(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn listar_ativas(&self) -> Result<Vec<Model>, String> {
        loja::Entity::find()
            .filter(loja::Column::Ativa.eq(true))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_criador(&self, admin_uuid: Uuid) -> Result<Vec<Model>, String> {
        loja::Entity::find()
            .filter(loja::Column::CriadoPor.eq(SeaUuid::from(admin_uuid)))
            .order_by_desc(loja::Column::CriadoEm)
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn pesquisar(&self, termo: &str) -> Result<Vec<Model>, String> {
        // Filter in memory - get all and filter in Rust
        // A proper implementation would use sea_orm::Condition with Like
        let all_lojas = self.listar_todos().await?;
        let termo_lower = termo.to_lowercase();
        let filtradas = all_lojas.into_iter()
            .filter(|l| {
                l.nome.to_lowercase().contains(&termo_lower) ||
                l.slug.to_lowercase().contains(&termo_lower) ||
                l.descricao.as_ref().map(|d| d.to_lowercase().contains(&termo_lower)).unwrap_or(false) ||
                l.email.to_lowercase().contains(&termo_lower)
            })
            .collect();
        Ok(filtradas)
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for LojaRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        loja::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Loja" 
    }
    
    fn entity_gender_suffix(&self) -> &'static str { 
        "a" 
    }



    async fn listar_todos_por_loja(&self, _: Uuid) -> Result<Vec<Model>, String> {
        Err("não se aplica".into())
    }
}
