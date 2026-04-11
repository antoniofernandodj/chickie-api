use async_trait::async_trait;
use uuid::Uuid;
use crate::models::Adicional;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait AdicionalRepositoryPort: Send + Sync {
    async fn criar(&self, adicional: &Adicional) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Adicional>>;
    async fn listar_todos(&self) -> DomainResult<Vec<Adicional>>;
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Adicional>>;
    async fn listar_disponiveis(&self, loja_uuid: Uuid) -> DomainResult<Vec<Adicional>>;
    async fn atualizar(&self, adicional: Adicional) -> DomainResult<()>;
    async fn atualizar_disponibilidade(&self, uuid: Uuid, disponivel: bool) -> DomainResult<()>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
}
