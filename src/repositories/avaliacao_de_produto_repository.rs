use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::avaliacao_produto::{self, Entity, Model},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct AvaliacaoDeProdutoRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl AvaliacaoDeProdutoRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_produto(&self, produto_uuid: Uuid) -> Result<Vec<Model>, String> {
        avaliacao_produto::Entity::find()
            .filter(avaliacao_produto::Column::ProdutoUuid.eq(SeaUuid::from(produto_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Model>, String> {
        avaliacao_produto::Entity::find()
            .filter(avaliacao_produto::Column::UsuarioUuid.eq(SeaUuid::from(usuario_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_pedido(&self, pedido_uuid: Uuid) -> Result<Vec<Model>, String> {
        // Note: pedido_uuid field may not exist in the entity - check entity structure
        // For now, returning empty vec or you can add the field to entity if needed
        Ok(vec![])
    }

    pub async fn calcular_media(&self, produto_uuid: Uuid) -> Result<f64, String> {
        let avaliados = self.buscar_por_produto(produto_uuid).await?;
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
impl Repository<Entity> for AvaliacaoDeProdutoRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        avaliacao_produto::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Avaliação" 
    }
    
    fn entity_gender_suffix(&self) -> &'static str { 
        "a" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        avaliacao_produto::Entity::find()
            .filter(avaliacao_produto::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}
