use async_trait::async_trait;
use uuid::Uuid;
use crate::models::Funcionario;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait FuncionarioRepositoryPort: Send + Sync {
    async fn criar(&self, funcionario: &Funcionario) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Funcionario>>;
    async fn buscar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Funcionario>>;
    async fn atualizar(&self, funcionario: Funcionario) -> DomainResult<()>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
}
