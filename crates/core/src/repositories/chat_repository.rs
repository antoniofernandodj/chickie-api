use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use std::sync::Arc;
use crate::models::{MensagemChat, CreateMensagemRequest, Model};
use crate::ports::ChatRepositoryPort;
use crate::repositories::Repository;
use crate::domain::errors::{DomainError, DomainResult};

pub struct ChatRepository {
    pool: Arc<PgPool>,
}

impl ChatRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<MensagemChat> for ChatRepository {
    fn table_name(&self) -> &'static str { "mensagens_chat" }
    fn entity_name(&self) -> &'static str { "mensagem de chat" }
    fn pool(&self) -> &PgPool { &self.pool }
    fn entity_gender_suffix(&self) -> &'static str { "a" }

    async fn criar(&self, item: &MensagemChat) -> Result<Uuid, String> {
        let query = "
            INSERT INTO mensagens_chat (pedido_uuid, loja_uuid, usuario_uuid, remetente_uuid, texto)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING uuid
        ";
        sqlx::query_scalar(query)
            .bind(item.pedido_uuid)
            .bind(item.loja_uuid)
            .bind(item.usuario_uuid)
            .bind(item.remetente_uuid)
            .bind(&item.texto)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, item: MensagemChat) -> Result<(), String> {
        let query = "
            UPDATE mensagens_chat
            SET lida = $1
            WHERE uuid = $2
        ";
        sqlx::query(query)
            .bind(item.lida)
            .bind(item.uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<MensagemChat>, String> {
        let query = "SELECT * FROM mensagens_chat WHERE loja_uuid = $1 ORDER BY criado_em ASC";
        sqlx::query_as::<_, MensagemChat>(query)
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait]
impl ChatRepositoryPort for ChatRepository {
    async fn listar_por_pedido(&self, pedido_uuid: Uuid) -> DomainResult<Vec<MensagemChat>> {
        let query = "SELECT * FROM mensagens_chat WHERE pedido_uuid = $1 ORDER BY criado_em ASC";
        sqlx::query_as::<_, MensagemChat>(query)
            .bind(pedido_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))
    }

    async fn listar_por_loja_usuario(&self, loja_uuid: Uuid, usuario_uuid: Uuid) -> DomainResult<Vec<MensagemChat>> {
        let query = "SELECT * FROM mensagens_chat WHERE loja_uuid = $1 AND usuario_uuid = $2 ORDER BY criado_em ASC";
        sqlx::query_as::<_, MensagemChat>(query)
            .bind(loja_uuid)
            .bind(usuario_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))
    }

    async fn criar_mensagem(&self, req: CreateMensagemRequest, remetente_uuid: Uuid) -> DomainResult<MensagemChat> {
        let query = "
            INSERT INTO mensagens_chat (pedido_uuid, loja_uuid, usuario_uuid, remetente_uuid, texto)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
        ";
        sqlx::query_as::<_, MensagemChat>(query)
            .bind(req.pedido_uuid)
            .bind(req.loja_uuid)
            .bind(req.usuario_uuid)
            .bind(remetente_uuid)
            .bind(req.texto)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))
    }

    async fn marcar_como_lida(&self, mensagem_uuid: Uuid) -> DomainResult<()> {
        let query = "UPDATE mensagens_chat SET lida = TRUE WHERE uuid = $1";
        sqlx::query(query)
            .bind(mensagem_uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))
            .map(|_| ())
    }
}
