use std::sync::Arc;

use sqlx::{postgres::PgPool, Acquire};
use uuid::Uuid;
use crate::ports::OrdemCategoriaRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct CategoriaOrdemRepository { pool: Arc<PgPool> }

impl CategoriaOrdemRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }
}

#[async_trait::async_trait]
impl OrdemCategoriaRepositoryPort for CategoriaOrdemRepository {
    async fn definir_ordens(&self, loja_uuid: Uuid, ordens: Vec<(Uuid, i32)>) -> DomainResult<()> {
        let mut conn = self.pool.acquire().await.map_err(|e| DomainError::Internal(e.to_string()))?;
        let mut tx = conn.begin().await.map_err(|e| DomainError::Internal(e.to_string()))?;

        for (categoria_uuid, ordem) in ordens {
            sqlx::query("
                INSERT INTO ordem_categorias_de_produtos (loja_uuid, categoria_uuid, ordem)
                VALUES ($1, $2, $3)
                ON CONFLICT (loja_uuid, categoria_uuid) DO UPDATE SET ordem = EXCLUDED.ordem
            ")
            .bind(loja_uuid)
            .bind(categoria_uuid)
            .bind(ordem)
            .execute(&mut *tx)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        }

        tx.commit().await.map_err(|e| DomainError::Internal(e.to_string()))
    }

    async fn proxima_ordem(&self, loja_uuid: Uuid) -> DomainResult<i32> {
        let max: Option<i32> = sqlx::query_scalar(
            "SELECT MAX(ordem) FROM ordem_categorias_de_produtos WHERE loja_uuid = $1"
        )
        .bind(loja_uuid)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(max.unwrap_or(0) + 1)
    }
}
