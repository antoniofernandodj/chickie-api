use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::PgPool;
use crate::models::Loja;
use crate::repositories::{LojaRepository, Repository};
use crate::domain::errors::{DomainError, DomainResult};
use crate::ports::LojaRepositoryPort;

pub struct LojaRepositoryAdapter {
    inner: Arc<LojaRepository>,
}

impl LojaRepositoryAdapter {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { inner: Arc::new(LojaRepository::new(pool)) }
    }

    pub fn from_repo(repo: Arc<LojaRepository>) -> Self {
        Self { inner: repo }
    }
}

#[async_trait]
impl LojaRepositoryPort for LojaRepositoryAdapter {
    async fn criar(&self, entity: &Loja) -> DomainResult<Uuid> {
        Repository::criar(&*self.inner, entity).await.map_err(|e| DomainError::Internal(e))
    }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Loja>> {
        Repository::buscar_por_uuid(&*self.inner, uuid).await.map_err(|e| DomainError::Internal(e))
    }

    async fn buscar_por_slug(&self, slug: &str) -> DomainResult<Option<Loja>> {
        self.inner.buscar_por_slug(slug).await.map_err(|e| DomainError::Internal(e))
    }

    async fn listar_todos(&self) -> DomainResult<Vec<Loja>> {
        Repository::listar_todos(&*self.inner).await.map_err(|e| DomainError::Internal(e))
    }

    async fn buscar_por_criador(&self, criador_uuid: Uuid) -> DomainResult<Vec<Loja>> {
        self.inner.buscar_por_criador(criador_uuid).await.map_err(|e| DomainError::Internal(e))
    }

    async fn pesquisar(&self, termo: &str) -> DomainResult<Vec<Loja>> {
        self.inner.pesquisar(termo).await.map_err(|e| DomainError::Internal(e))
    }
}
