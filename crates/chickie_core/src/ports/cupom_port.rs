use async_trait::async_trait;
use uuid::Uuid;
use crate::models::Cupom;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait CupomRepositoryPort: Send + Sync {
    async fn criar(&self, cupom: &Cupom) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Cupom>>;
    async fn buscar_por_codigo(&self, codigo: &str, loja_uuid: Uuid) -> DomainResult<Option<Cupom>>;
    async fn buscar_ativos(&self, loja_uuid: Uuid) -> DomainResult<Vec<Cupom>>;
    async fn listar_todos(&self) -> DomainResult<Vec<Cupom>>;
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Cupom>>;
    async fn atualizar(&self, cupom: Cupom) -> DomainResult<()>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
}
