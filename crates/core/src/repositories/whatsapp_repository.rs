use std::sync::Arc;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::WhatsAppBinding;
use crate::ports::WhatsAppRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};
use crate::repositories::Repository;

pub struct WhatsAppRepository {
    pool: Arc<PgPool>,
}

impl WhatsAppRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<WhatsAppBinding> for WhatsAppRepository {
    fn pool(&self) -> &PgPool {
        &self.pool
    }
    fn table_name(&self) -> &'static str {
        "user_whatsapp_bindings"
    }
    fn entity_name(&self) -> &'static str {
        "whatsapp_binding"
    }

    async fn criar(&self, item: &WhatsAppBinding) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO user_whatsapp_bindings (uuid, user_id, phone_number, verified, verification_code_hash, verification_expires_at)
               VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(item.uuid)
        .bind(item.user_id)
        .bind(&item.phone_number)
        .bind(item.verified)
        .bind(&item.verification_code_hash)
        .bind(item.verification_expires_at)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(item.uuid)
    }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<WhatsAppBinding>, String> {
        sqlx::query_as::<_, WhatsAppBinding>(
            "SELECT * FROM user_whatsapp_bindings WHERE uuid = $1"
        )
        .bind(uuid)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn listar_todos(&self) -> Result<Vec<WhatsAppBinding>, String> {
        sqlx::query_as::<_, WhatsAppBinding>(
            "SELECT * FROM user_whatsapp_bindings"
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, item: WhatsAppBinding) -> Result<(), String> {
        sqlx::query(
            "UPDATE user_whatsapp_bindings SET verified = $2, verification_code_hash = $3, verification_expires_at = $4, atualizado_em = NOW()
               WHERE uuid = $1",
        )
        .bind(item.uuid)
        .bind(item.verified)
        .bind(&item.verification_code_hash)
        .bind(item.verification_expires_at)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        sqlx::query("DELETE FROM user_whatsapp_bindings WHERE uuid = $1")
            .bind(uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn listar_todos_por_loja(&self, _loja_uuid: Uuid) -> Result<Vec<WhatsAppBinding>, String> {
        Ok(vec![])
    }
}

#[async_trait]
impl WhatsAppRepositoryPort for WhatsAppRepository {
    async fn criar_binding(&self, binding: &WhatsAppBinding) -> DomainResult<Uuid> {
        <Self as Repository<WhatsAppBinding>>::criar(self, binding)
            .await
            .map_err(|e| DomainError::Internal(e))
    }

    async fn buscar_binding_por_phone(&self, phone: &str) -> DomainResult<Option<WhatsAppBinding>> {
        sqlx::query_as::<_, WhatsAppBinding>(
            "SELECT * FROM user_whatsapp_bindings WHERE phone_number = $1"
        )
        .bind(phone)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
    }

    async fn buscar_binding_por_user(&self, user_id: Uuid) -> DomainResult<Option<WhatsAppBinding>> {
        sqlx::query_as::<_, WhatsAppBinding>(
            "SELECT * FROM user_whatsapp_bindings WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
    }

    async fn atualizar_binding(&self, binding: WhatsAppBinding) -> DomainResult<()> {
        <Self as Repository<WhatsAppBinding>>::atualizar(self, binding)
            .await
            .map_err(|e| DomainError::Internal(e))
    }

    async fn deletar_binding(&self, uuid: Uuid) -> DomainResult<()> {
        <Self as Repository<WhatsAppBinding>>::deletar(self, uuid)
            .await
            .map_err(|e| DomainError::Internal(e))
    }

    async fn registrar_mensagem_processada(&self, message_sid: &str) -> DomainResult<()> {
        sqlx::query(
            "INSERT INTO processed_twilio_messages (message_sid) VALUES ($1) ON CONFLICT DO NOTHING",
        )
        .bind(message_sid)
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn ja_processada(&self, message_sid: &str) -> DomainResult<bool> {
        let exists = sqlx::query(
            "SELECT 1 FROM processed_twilio_messages WHERE message_sid = $1",
        )
        .bind(message_sid)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(exists.is_some())
    }
}
