use async_trait::async_trait;
use uuid::Uuid;
use crate::models::LojaFavorita;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait LojaFavoritaRepositoryPort: Send + Sync {
    async fn criar(&self, favorita: &LojaFavorita) -> DomainResult<Uuid>;
    async fn listar_por_usuario(&self, usuario_uuid: Uuid) -> DomainResult<Vec<LojaFavorita>>;
    async fn buscar_por_usuario_e_loja(&self, usuario_uuid: Uuid, loja_uuid: Uuid) -> DomainResult<Option<LojaFavorita>>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
}
