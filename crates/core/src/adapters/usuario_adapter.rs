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
        self.inner.criar(entity).await.map_err(|e| DomainError::Internal(e))
    }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Usuario>> {
        self.inner.buscar_por_uuid(uuid).await.map_err(|e| DomainError::Internal(e))
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
        self.inner.listar_todos().await.map_err(|e| DomainError::Internal(e))
    }

    async fn atualizar(&self, entity: Usuario) -> DomainResult<()> {
        self.inner.atualizar(entity).await.map_err(|e| DomainError::Internal(e))
    }

    async fn marcar_primeiro_acesso(&self, uuid: Uuid) -> DomainResult<()> {
        self.inner.marcar_primeiro_acesso(uuid).await.map_err(|e| DomainError::Internal(e))
    }
}
