use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait PreCadastroPort: Send + Sync {
    async fn salvar(&self, token: &str, dados: serde_json::Value, expira_em: DateTime<Utc>) -> DomainResult<()>;
    async fn buscar(&self, token: &str) -> DomainResult<Option<serde_json::Value>>;
    async fn remover(&self, token: &str) -> DomainResult<()>;
    async fn limpar_expirados(&self) -> DomainResult<u64>;
}
