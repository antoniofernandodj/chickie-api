use async_trait::async_trait;
use uuid::Uuid;
use crate::models::Loja;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait LojaRepositoryPort: Send + Sync {
    async fn criar(&self, entity: &Loja) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Loja>>;
    async fn buscar_por_slug(&self, slug: &str) -> DomainResult<Option<Loja>>;
    async fn listar_todos(&self) -> DomainResult<Vec<Loja>>;
    async fn buscar_por_criador(&self, criador_uuid: Uuid) -> DomainResult<Vec<Loja>>;
    async fn pesquisar(&self, termo: &str) -> DomainResult<Vec<Loja>>;
}
