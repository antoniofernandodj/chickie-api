use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::PgPool;
use crate::models::Usuario;
use crate::repositories::{UsuarioRepository, Repository};
use crate::domain::errors::{DomainError, DomainResult};
use crate::ports::UsuarioRepositoryPort;

pub struct UsuarioRepositoryAdapter {
    inner: Arc<UsuarioRepository>,
}

impl UsuarioRepositoryAdapter {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { inner: Arc::new(UsuarioRepository::new(pool)) }
    }

    pub fn from_repo(repo: Arc<UsuarioRepository>) -> Self {
        Self { inner: repo }
    }
}

#[async_trait]
impl UsuarioRepositoryPort for UsuarioRepositoryAdapter {
    async fn criar(&self, entity: &Usuario) -> DomainResult<Uuid> {
        Repository::criar(&*self.inner, entity).await.map_err(|e| DomainError::Internal(e))
    }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Usuario>> {
        Repository::buscar_por_uuid(&*self.inner, uuid).await.map_err(|e| DomainError::Internal(e))
    }

    async fn buscar_por_email(&self, email: &str) -> DomainResult<Option<Usuario>> {
        self.inner.buscar_por_email(email).await.map_err(|e| DomainError::Internal(e))
    }

    async fn buscar_por_username(&self, username: &str) -> DomainResult<Option<Usuario>> {
        self.inner.buscar_por_username(username).await.map_err(|e| DomainError::Internal(e))
    }

    async fn buscar_por_celular(&self, celular: &str) -> DomainResult<Option<Usuario>> {
        self.inner.buscar_por_celular(celular).await.map_err(|e| DomainError::Internal(e))
    }

    async fn listar_todos(&self) -> DomainResult<Vec<Usuario>> {
        Repository::listar_todos(&*self.inner).await.map_err(|e| DomainError::Internal(e))
    }

    async fn atualizar(&self, entity: Usuario) -> DomainResult<()> {
        Repository::atualizar(&*self.inner, entity).await.map_err(|e| DomainError::Internal(e))
    }

    async fn marcar_primeiro_acesso(&self, uuid: Uuid) -> DomainResult<()> {
        self.inner.marcar_primeiro_acesso(uuid).await.map_err(|e| DomainError::Internal(e))
    }

    async fn marcar_para_remocao(&self, uuid: Uuid) -> DomainResult<()> {
        self.inner.marcar_para_remocao(uuid).await.map_err(|e| DomainError::Internal(e))
    }

    async fn desmarcar_remocao(&self, uuid: Uuid) -> DomainResult<()> {
        self.inner.desmarcar_remocao(uuid).await.map_err(|e| DomainError::Internal(e))
    }

    async fn marcar_como_deletado(&self, uuid: Uuid) -> DomainResult<()> {
        self.inner.marcar_como_deletado(uuid).await.map_err(|e| DomainError::Internal(e))
    }

    async fn alterar_ativo(&self, uuid: Uuid, ativo: bool) -> DomainResult<()> {
        self.inner.alterar_ativo(uuid, ativo).await.map_err(|e| DomainError::Internal(e))
    }

    async fn toggle_bloqueado(&self, uuid: Uuid) -> DomainResult<bool> {
        self.inner.toggle_bloqueado(uuid).await.map_err(|e| DomainError::Internal(e))
    }

    async fn listar_pendentes_remocao(&self) -> DomainResult<Vec<Usuario>> {
        self.inner.listar_pendentes_remocao().await.map_err(|e| DomainError::Internal(e))
    }

    async fn deletar_pendentes_antigos(&self, limite: chrono::DateTime<chrono::Utc>) -> DomainResult<u64> {
        self.inner.deletar_pendentes_antigos(limite).await.map_err(|e| DomainError::Internal(e))
    }
    async fn salvar_asaas_customer_id(&self, uuid: Uuid, customer_id: &str) -> DomainResult<()> {
        self.inner.salvar_asaas_customer_id(uuid, customer_id).await.map_err(|e| DomainError::Internal(e))
    }
}
