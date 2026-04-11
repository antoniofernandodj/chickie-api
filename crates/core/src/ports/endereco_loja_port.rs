use async_trait::async_trait;
use uuid::Uuid;
use crate::models::EnderecoLoja;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait EnderecoLojaRepositoryPort: Send + Sync {
    async fn criar(&self, endereco: &EnderecoLoja) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<EnderecoLoja>>;
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<EnderecoLoja>>;
    async fn atualizar(&self, endereco: EnderecoLoja) -> DomainResult<()>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
}
