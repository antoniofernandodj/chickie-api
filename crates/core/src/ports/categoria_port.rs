use async_trait::async_trait;
use uuid::Uuid;
use crate::models::CategoriaProdutos;
use crate::domain:: errors::DomainResult;

#[async_trait]
pub trait CategoriaRepositoryPort: Send + Sync {
    async fn criar(&self, entity: &CategoriaProdutos) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<CategoriaProdutos>>;
    async fn listar_todos(&self) -> DomainResult<Vec<CategoriaProdutos>>;
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<CategoriaProdutos>>;
    async fn atualizar(&self, entity: CategoriaProdutos) -> DomainResult<()>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
    async fn contar_produtos(&self, categoria_uuid: Uuid) -> DomainResult<i64>;
}
