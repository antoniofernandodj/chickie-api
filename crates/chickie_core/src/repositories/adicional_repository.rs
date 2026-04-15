use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Adicional, Model}, repositories::Repository};
use crate::ports::AdicionalRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct AdicionalRepository { pool: Arc<PgPool> }

impl AdicionalRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Adicional>, String> {
        sqlx::query_as::<_, Adicional>("SELECT * FROM adicionais WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Adicional>, String> {
        sqlx::query_as::<_, Adicional>(
            "SELECT * FROM adicionais WHERE loja_uuid = $1 AND disponivel = true"
        )
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn atualizar_disponibilidade(&self, uuid: Uuid, disponivel: bool) -> Result<(), String> {
        let result = sqlx::query("UPDATE adicionais SET disponivel = $1 WHERE uuid = $2")
            .bind(disponivel)
            .bind(uuid)
            .execute(self.pool())
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Adicional não encontrado".to_string())
        } else {
            Ok(())
        }
    }
}

#[async_trait::async_trait]
impl Repository<Adicional> for AdicionalRepository {
    fn table_name(&self) -> &'static str { "adicionais" }
    fn entity_name(&self) -> &'static str { "Adicional" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &Adicional) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO adicionais (uuid, loja_uuid, nome, descricao, preco, disponivel)
            VALUES ($1, $2, $3, $4, $5, $6);
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.preco)
        .bind(item.disponivel)
        .execute(self.pool())
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Adicional) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE adicionais SET loja_uuid = $1, nome = $2, descricao = $3, preco = $4, disponivel = $5
            WHERE uuid = $6
        ")
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.preco)
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

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Adicional>, String> {
        sqlx::query_as::<_, Adicional>("SELECT * FROM adicionais WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl AdicionalRepositoryPort for AdicionalRepository {
    async fn criar(&self, adicional: &Adicional) -> DomainResult<Uuid> {
        <Self as Repository<Adicional>>::criar(self, adicional).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Adicional>> {
        <Self as Repository<Adicional>>::buscar_por_uuid(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_todos(&self) -> DomainResult<Vec<Adicional>> {
        <Self as Repository<Adicional>>::listar_todos(self).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Adicional>> {
        self.buscar_por_loja(loja_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_disponiveis(&self, loja_uuid: Uuid) -> DomainResult<Vec<Adicional>> {
        self.buscar_disponiveis(loja_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn atualizar(&self, adicional: Adicional) -> DomainResult<()> {
        <Self as Repository<Adicional>>::atualizar(self, adicional).await.map_err(|e| DomainError::Internal(e))
    }
    async fn atualizar_disponibilidade(&self, uuid: Uuid, disponivel: bool) -> DomainResult<()> {
        self.atualizar_disponibilidade(uuid, disponivel).await.map_err(|e| DomainError::Internal(e))
    }
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()> {
        <Self as Repository<Adicional>>::deletar(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
}
