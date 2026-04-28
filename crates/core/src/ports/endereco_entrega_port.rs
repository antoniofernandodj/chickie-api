use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;
use crate::models::EnderecoEntrega;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait EnderecoEntregaRepositoryPort: Send + Sync {
    async fn criar(&self, endereco: &EnderecoEntrega) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<EnderecoEntrega>>;
    async fn buscar_por_pedido(&self, pedido_uuid: Uuid) -> DomainResult<Option<EnderecoEntrega>>;
    async fn buscar_por_pedidos(&self, pedido_uuids: &[Uuid]) -> DomainResult<HashMap<Uuid, EnderecoEntrega>>;
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<EnderecoEntrega>>;
    async fn atualizar(&self, endereco: EnderecoEntrega) -> DomainResult<()>;
}
