use async_trait::async_trait;
use uuid::Uuid;
use crate::models::Usuario;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait UsuarioRepositoryPort: Send + Sync {
    async fn criar(&self, entity: &Usuario) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Usuario>>;
    async fn buscar_por_email(&self, email: &str) -> DomainResult<Option<Usuario>>;
    async fn buscar_por_username(&self, username: &str) -> DomainResult<Option<Usuario>>;
    async fn buscar_por_celular(&self, celular: &str) -> DomainResult<Option<Usuario>>;
    async fn listar_todos(&self) -> DomainResult<Vec<Usuario>>;
    async fn atualizar(&self, entity: Usuario) -> DomainResult<()>;
    async fn marcar_primeiro_acesso(&self, uuid: Uuid) -> DomainResult<()>;

    // Soft delete methods
    async fn marcar_para_remocao(&self, uuid: Uuid) -> DomainResult<()>;
    async fn desmarcar_remocao(&self, uuid: Uuid) -> DomainResult<()>;
    async fn marcar_como_deletado(&self, uuid: Uuid) -> DomainResult<()>;
    async fn alterar_ativo(&self, uuid: Uuid, ativo: bool) -> DomainResult<()>;

    // Listar usuários pendentes de remoção (para o scheduler)
    async fn listar_pendentes_remocao(&self) -> DomainResult<Vec<Usuario>>;
}
