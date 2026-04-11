use async_trait::async_trait;
use uuid::Uuid;
use crate::models::AvaliacaoDeLoja;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait AvaliacaoDeLojaRepositoryPort: Send + Sync {
    async fn criar(&self, avaliacao: &AvaliacaoDeLoja) -> DomainResult<Uuid>;
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<AvaliacaoDeLoja>>;
}
