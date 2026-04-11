use async_trait::async_trait;
use uuid::Uuid;
use crate::models::Promocao;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait PromocaoRepositoryPort: Send + Sync {
    async fn criar(&self, promocao: &Promocao) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Promocao>>;
    async fn listar_todos(&self) -> DomainResult<Vec<Promocao>>;
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Promocao>>;
    async fn atualizar(&self, promocao: Promocao) -> DomainResult<()>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
}
