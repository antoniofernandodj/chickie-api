use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Entregador, Model}, repositories::Repository};
use crate::ports::EntregadorRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct EntregadorRepository { pool: Arc<PgPool> }

#[allow(dead_code)]
impl EntregadorRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Entregador>, String> {
        sqlx::query_as::<_, Entregador>("SELECT * FROM entregadores WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Entregador>, String> {
        sqlx::query_as::<_, Entregador>("SELECT * FROM entregadores WHERE loja_uuid = $1 AND disponivel = true")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Option<Entregador>, String> {
        sqlx::query_as::<_, Entregador>("SELECT * FROM entregadores WHERE usuario_uuid = $1")
        .bind(usuario_uuid)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entregador> for EntregadorRepository {
    fn table_name(&self) -> &'static str { "entregadores" }
    fn entity_name(&self) -> &'static str { "Entregador" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &Entregador) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO entregadores (uuid, loja_uuid, usuario_uuid, veiculo, placa, disponivel)
            VALUES ($1, $2, $3, $4, $5, $6)
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(item.usuario_uuid)
        .bind(&item.veiculo)
        .bind(&item.placa)
        .bind(item.disponivel)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Entregador) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE entregadores SET loja_uuid = $1, usuario_uuid = $2, veiculo = $3, placa = $4, disponivel = $5
            WHERE uuid = $6
        ")
        .bind(item.loja_uuid)
        .bind(item.usuario_uuid)
        .bind(&item.veiculo)
        .bind(&item.placa)
        .bind(item.disponivel)
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

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Entregador>, String> {
        sqlx::query_as::<_, Entregador>("SELECT * FROM entregadores WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }

}

#[async_trait::async_trait]
impl EntregadorRepositoryPort for EntregadorRepository {
    async fn criar(&self, entregador: &Entregador) -> DomainResult<Uuid> {
        <Self as Repository<Entregador>>::criar(self, entregador).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Entregador>> {
        <Self as Repository<Entregador>>::buscar_por_uuid(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Entregador>> {
        self.buscar_por_loja(loja_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> DomainResult<Vec<Entregador>> {
        self.buscar_disponiveis(loja_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn atualizar(&self, entregador: Entregador) -> DomainResult<()> {
        <Self as Repository<Entregador>>::atualizar(self, entregador).await.map_err(|e| DomainError::Internal(e))
    }
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()> {
        <Self as Repository<Entregador>>::deletar(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
}
