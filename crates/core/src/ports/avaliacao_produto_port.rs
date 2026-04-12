use async_trait::async_trait;
use uuid::Uuid;
use crate::models::AvaliacaoDeProduto;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait AvaliacaoDeProdutoRepositoryPort: Send + Sync {
    async fn criar(&self, avaliacao: &AvaliacaoDeProduto) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<AvaliacaoDeProduto>>;
    async fn listar_por_produto(&self, produto_uuid: Uuid) -> DomainResult<Vec<AvaliacaoDeProduto>>;
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<AvaliacaoDeProduto>>;
    async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> DomainResult<Vec<AvaliacaoDeProduto>>;
    async fn buscar_por_usuario_e_produto(&self, usuario_uuid: Uuid, produto_uuid: Uuid) -> DomainResult<Option<AvaliacaoDeProduto>>;
    async fn atualizar(&self, avaliacao: AvaliacaoDeProduto) -> DomainResult<()>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
}
