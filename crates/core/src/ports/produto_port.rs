use async_trait::async_trait;
use uuid::Uuid;
use crate::models::Produto;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait ProdutoRepositoryPort: Send + Sync {
    async fn criar(&self, produto: &Produto) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Produto>>;
    async fn listar_todos(&self) -> DomainResult<Vec<Produto>>;
    async fn listar_por_categoria(&self, loja_uuid: Uuid, categoria_uuid: Uuid) -> DomainResult<Vec<Produto>>;
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Produto>>;
    async fn atualizar(&self, produto: Produto) -> DomainResult<()>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
    async fn atualizar_disponibilidade(&self, uuid: Uuid, disponivel: bool) -> DomainResult<()>;
    async fn atualizar_imagem_url(&self, uuid: Uuid, imagem_url: &str) -> DomainResult<()>;
}
