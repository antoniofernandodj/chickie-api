use async_trait::async_trait;
use uuid::Uuid;
use crate::models::Entregador;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait EntregadorRepositoryPort: Send + Sync {
    async fn criar(&self, entregador: &Entregador) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Entregador>>;
    async fn buscar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Entregador>>;
    async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> DomainResult<Vec<Entregador>>;
    async fn atualizar(&self, entregador: Entregador) -> DomainResult<()>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
}
