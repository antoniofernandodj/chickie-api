use std::sync::Arc;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::PgPool;

use crate::domain::errors::{DomainError, DomainResult};
use crate::ports::PreCadastroPort;

pub struct PreCadastroRepository {
    pool: Arc<PgPool>,
}

impl PreCadastroRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PreCadastroPort for PreCadastroRepository {
    async fn salvar(&self, token: &str, dados: Value, expira_em: DateTime<Utc>) -> DomainResult<()> {
        sqlx::query(
            "INSERT INTO pre_cadastro (token, dados, expira_em)
             VALUES ($1, $2, $3)
             ON CONFLICT (token) DO UPDATE
             SET dados = EXCLUDED.dados, expira_em = EXCLUDED.expira_em"
        )
        .bind(token)
        .bind(dados)
        .bind(expira_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn buscar(&self, token: &str) -> DomainResult<Option<Value>> {
        let row: Option<(Value,)> = sqlx::query_as(
            "SELECT dados FROM pre_cadastro WHERE token = $1 AND expira_em > NOW()"
        )
        .bind(token)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(row.map(|(v,)| v))
    }

    async fn remover(&self, token: &str) -> DomainResult<()> {
        sqlx::query("DELETE FROM pre_cadastro WHERE token = $1")
            .bind(token)
            .execute(&*self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn limpar_expirados(&self) -> DomainResult<u64> {
        let result = sqlx::query("DELETE FROM pre_cadastro WHERE expira_em <= NOW()")
            .execute(&*self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(result.rows_affected())
    }
}
