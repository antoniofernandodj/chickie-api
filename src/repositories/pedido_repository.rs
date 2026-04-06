use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait,QueryOrder,TransactionTrait};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::{
        pedido::{self, Entity, Model},
        item_pedido::{self, Entity as ItemEntity, Model as ItemModel},
        partes_item_pedido::{self, Entity as PartesEntity, Model as PartesModel},
    },
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

// Note: This is a simplified version. The full PedidoRepository with nested entities
// will need custom handling as Pedido has complex nested structures (itens, partes, adicionais)
pub struct PedidoRepository { 
    db: Arc<DatabaseConnection> 
}

#[allow(dead_code)]
impl PedidoRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { 
        Self { db } 
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Model>, String> {
        pedido::Entity::find()
            .filter(pedido::Column::UsuarioUuid.eq(SeaUuid::from(usuario_uuid)))
            .order_by_desc(pedido::Column::CriadoEm)
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        pedido::Entity::find()
            .filter(pedido::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .order_by_desc(pedido::Column::CriadoEm)
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_status(&self, status: String) -> Result<Vec<Model>, String> {
        pedido::Entity::find()
            .filter(pedido::Column::Status.eq(status))
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_pendentes(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        pedido::Entity::find()
            .filter(pedido::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .filter(
                sea_orm::Condition::any()
                    .add(pedido::Column::Status.eq("em_preparo"))
                    .add(pedido::Column::Status.eq("criado"))
            )
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for PedidoRepository {
    fn db(&self) -> &DatabaseConnection { 
        &*self.db 
    }
    
    fn entity(&self) -> Entity { 
        pedido::Entity 
    }

    fn entity_name(&self) -> &'static str { 
        "Pedido" 
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
