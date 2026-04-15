use async_trait::async_trait;
use uuid::Uuid;
use crate::models::EnderecoUsuario;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait EnderecoUsuarioRepositoryPort: Send + Sync {
    async fn criar(&self, endereco: &EnderecoUsuario) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<EnderecoUsuario>>;
    async fn listar_por_usuario(&self, usuario_uuid: Uuid) -> DomainResult<Vec<EnderecoUsuario>>;
    async fn atualizar(&self, endereco: EnderecoUsuario) -> DomainResult<()>;
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()>;
}
