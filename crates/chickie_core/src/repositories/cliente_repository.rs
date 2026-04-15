use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Cliente, Model}, repositories::Repository};
use crate::ports::ClienteRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct ClienteRepository { pool: Arc<PgPool> }

#[allow(dead_code)]
impl ClienteRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Cliente>, String> {
        sqlx::query_as::<_, Cliente>("SELECT * FROM clientes WHERE usuario_uuid = $1")
        .bind(usuario_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Cliente>, String> {
        sqlx::query_as::<_, Cliente>("SELECT * FROM clientes WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Cliente> for ClienteRepository {
    fn table_name(&self) -> &'static str { "clientes" }
    fn entity_name(&self) -> &'static str { "Cliente" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &Cliente) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO clientes (uuid, usuario_uuid, loja_uuid)
            VALUES ($1, $2, $3);
        ")
        .bind(item.uuid)
        .bind(item.usuario_uuid)
        .bind(item.loja_uuid)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Cliente) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query(
            "UPDATE clientes SET usuario_uuid = $1, loja_uuid = $2 WHERE uuid = $3"
        )
        .bind(item.usuario_uuid)
        .bind(item.loja_uuid)
        .bind(uuid)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err(format!("{} não encontrad{}", self.entity_name(), self.entity_gender_suffix()))
        } else {
            Ok(())
        }
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Cliente>, String> {
        sqlx::query_as::<_, Cliente>("SELECT * FROM clientes WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl ClienteRepositoryPort for ClienteRepository {
    async fn criar(&self, cliente: &Cliente) -> DomainResult<Uuid> {
        <Self as Repository<Cliente>>::criar(self, cliente).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Cliente>> {
        self.buscar_por_loja(loja_uuid).await.map_err(|e| DomainError::Internal(e))
    }
}
