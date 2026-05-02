use async_trait::async_trait;
use uuid::Uuid;
use crate::models::WhatsAppBinding;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait WhatsAppRepositoryPort: Send + Sync {
    async fn criar_binding(&self, binding: &WhatsAppBinding) -> DomainResult<Uuid>;
    async fn buscar_binding_por_phone(&self, phone: &str) -> DomainResult<Option<WhatsAppBinding>>;
    async fn buscar_binding_por_user(&self, user_id: Uuid) -> DomainResult<Option<WhatsAppBinding>>;
    async fn atualizar_binding(&self, binding: WhatsAppBinding) -> DomainResult<()>;
    async fn deletar_binding(&self, uuid: Uuid) -> DomainResult<()>;
    
    async fn registrar_mensagem_processada(&self, message_sid: &str) -> DomainResult<()>;
    async fn ja_processada(&self, message_sid: &str) -> DomainResult<bool>;
}
