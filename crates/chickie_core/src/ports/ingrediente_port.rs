use async_trait::async_trait;
use uuid::Uuid;
use crate::models::Ingrediente;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait IngredienteRepositoryPort: Send + Sync {
    async fn criar(&self, ingrediente: &Ingrediente) -> DomainResult<Uuid>;
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Ingrediente>>;
    async fn atualizar(&self, ingrediente: Ingrediente) -> DomainResult<()>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
}
