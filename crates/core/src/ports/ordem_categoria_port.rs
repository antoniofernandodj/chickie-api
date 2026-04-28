use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait OrdemCategoriaRepositoryPort: Send + Sync {
    /// Upsert de múltiplas ordens para uma loja numa transação.
    async fn definir_ordens(&self, loja_uuid: Uuid, ordens: Vec<(Uuid, i32)>) -> DomainResult<()>;
    /// Próximo valor de ordem disponível para a loja (MAX + 1).
    async fn proxima_ordem(&self, loja_uuid: Uuid) -> DomainResult<i32>;
}
