use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::avaliacao_loja::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct AvaliacaoDeLojaRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl AvaliacaoDeLojaRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        avaliacao_loja::Entity::find()
            .filter(avaliacao_loja::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Model>, String> {
        avaliacao_loja::Entity::find()
            .filter(avaliacao_loja::Column::UsuarioUuid.eq(SeaUuid::from(usuario_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn calcular_media(&self, loja_uuid: Uuid) -> Result<f64, String> {
        // Using raw SQL for aggregation as sea-orm doesn't have built-in AVG
        let avaliados = self.buscar_por_loja(loja_uuid).await?;
        if avaliados.is_empty() {
            return Ok(0.0);
        }
        
        let soma: f64 = avaliados.iter()
            .map(|a| a.nota.to_f64().unwrap_or(0.0))
            .sum();
        
        Ok(soma / avaliados.len() as f64)
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for AvaliacaoDeLojaRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        avaliacao_loja::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Avaliação" 
    }
    
    fn entity_gender_suffix(&self) -> &'static str { 
        "a" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
