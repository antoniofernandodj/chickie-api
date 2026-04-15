use uuid::Uuid;
use async_trait::async_trait;
use crate::domain::errors::DomainResult;

/// Repository port — generic over any entity T.
/// No sqlx, no database specifics. This is a pure interface.
#[async_trait]
pub trait RepositoryPort<T>: Send + Sync
where
    T: Send + Sync + Clone + 'static,
{
    async fn criar(&self, entity: &T) -> DomainResult<()>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<T>>;
    async fn listar_todos(&self) -> DomainResult<Vec<T>>;
    async fn atualizar(&self, entity: &T) -> DomainResult<()>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
}

/// Marker trait for entities that can be identified by UUID
pub trait Entity: Send + Sync + Clone + 'static {
    fn uuid(&self) -> Uuid;
    fn set_uuid(&mut self, uuid: Uuid);
}
