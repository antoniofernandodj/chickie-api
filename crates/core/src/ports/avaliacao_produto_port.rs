use async_trait::async_trait;
use uuid::Uuid;
use crate::models::AvaliacaoDeProduto;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait AvaliacaoDeProdutoRepositoryPort: Send + Sync {
    async fn criar(&self, avaliacao: &AvaliacaoDeProduto) -> DomainResult<Uuid>;
    async fn listar_por_produto(&self, produto_uuid: Uuid) -> DomainResult<Vec<AvaliacaoDeProduto>>;
}
