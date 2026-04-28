use async_trait::async_trait;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait EmailServicePort: Send + Sync {
    async fn enviar_verificacao_cadastro(&self, email: &str, nome: &str, token: &str) -> DomainResult<()>;
}
