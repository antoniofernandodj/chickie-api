use async_trait::async_trait;
use uuid::Uuid;
use crate::models::Cliente;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait ClienteRepositoryPort: Send + Sync {
    async fn criar(&self, cliente: &Cliente) -> DomainResult<Uuid>;
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Cliente>>;
}
