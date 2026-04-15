use async_trait::async_trait;
use uuid::Uuid;
use crate::models::HorarioFuncionamento;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait HorarioFuncionamentoRepositoryPort: Send + Sync {
    async fn adicionar_sem_sobrescrever(&self, horario: &HorarioFuncionamento) -> DomainResult<()>;
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<HorarioFuncionamento>>;
    async fn buscar_por_loja_e_dia(&self, loja_uuid: Uuid, dia: i32) -> DomainResult<Option<HorarioFuncionamento>>;
    async fn definir_ativo(&self, loja_uuid: Uuid, dia: i32, ativo: bool) -> DomainResult<()>;
    async fn deletar_por_dia(&self, loja_uuid: Uuid, dia: i32) -> DomainResult<()>;
}
