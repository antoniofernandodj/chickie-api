use async_trait::async_trait;
use uuid::Uuid;
use crate::models::{AvaliacaoDeLoja, AvaliacaoDeLojaComUsuario};
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait AvaliacaoDeLojaRepositoryPort: Send + Sync {
    async fn criar(&self, avaliacao: &AvaliacaoDeLoja) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<AvaliacaoDeLoja>>;
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<AvaliacaoDeLoja>>;
    async fn listar_por_loja_com_usuario(&self, loja_uuid: Uuid) -> DomainResult<Vec<AvaliacaoDeLojaComUsuario>>;
    async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> DomainResult<Vec<AvaliacaoDeLoja>>;
    async fn buscar_por_usuario_e_loja(&self, usuario_uuid: Uuid, loja_uuid: Uuid) -> DomainResult<Option<AvaliacaoDeLoja>>;
    async fn atualizar(&self, avaliacao: AvaliacaoDeLoja) -> DomainResult<()>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
}
