use sea_orm::{DatabaseConnection,EntityTrait,QueryFilter,ColumnTrait,QueryOrder,ActiveModelTrait,Set};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    entities::endereco_entrega::{self, Entity, Model, ActiveModel},
    repositories::Repository,
};
use sea_orm::prelude::Uuid as SeaUuid;

pub struct EnderecoEntregaRepository {
    db: Arc<DatabaseConnection>
}

impl EnderecoEntregaRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// Busca o endereco de entrega vinculado a um pedido especifico
    pub async fn buscar_por_pedido(&self, pedido_uuid: Uuid) -> Result<Option<Model>, String> {
        endereco_entrega::Entity::find()
            .filter(endereco_entrega::Column::PedidoUuid.eq(SeaUuid::from(pedido_uuid)))
            .one(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Busca enderecos de entrega por loja (util para relatorios/auditoria)
    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        endereco_entrega::Entity::find()
            .filter(endereco_entrega::Column::LojaUuid.eq(SeaUuid::from(loja_uuid)))
            .order_by_desc(endereco_entrega::Column::CriadoEm)
            .all(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Cria um endereco de entrega vinculado a um pedido (uso interno no fluxo de checkout)
    pub async fn criar_para_pedido(&self, endereco: &Model, pedido_uuid: Uuid, loja_uuid: Uuid) -> Result<Model, String> {
        let mut active: ActiveModel = endereco.clone().into();
        active.pedido_uuid = Set(SeaUuid::from(pedido_uuid));
        active.loja_uuid = Set(SeaUuid::from(loja_uuid));

        active.insert(&*self.db)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entity> for EnderecoEntregaRepository {
    fn db(&self) -> &DatabaseConnection {
        &*self.db
    }

    fn entity(&self) -> Entity {
        endereco_entrega::Entity
    }

    fn entity_name(&self) -> &'static str {
        "Endereco de entrega"
    }



    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Model>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
