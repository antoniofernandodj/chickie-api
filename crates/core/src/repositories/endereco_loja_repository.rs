use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{EnderecoLoja, Model}, repositories::Repository};
use crate::ports::EnderecoLojaRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct EnderecoLojaRepository { pool: Arc<PgPool> }

#[allow(dead_code)]
impl EnderecoLojaRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<EnderecoLoja>, String> {
        sqlx::query_as::<_, EnderecoLoja>("SELECT * FROM enderecos_loja WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<EnderecoLoja> for EnderecoLojaRepository {
    fn table_name(&self) -> &'static str { "enderecos_loja" }
    fn entity_name(&self) -> &'static str { "Endereço" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &EnderecoLoja) -> Result<Uuid, String> {
        let uuid = item.get_uuid();
        sqlx::query("
            INSERT INTO enderecos_loja (uuid, loja_uuid, cep, logradouro, numero, complemento, bairro, cidade, estado, latitude, longitude)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ")
        .bind(uuid)
        .bind(item.loja_uuid)
        .bind(&item.cep)
        .bind(&item.logradouro)
        .bind(&item.numero)
        .bind(&item.complemento)
        .bind(&item.bairro)
        .bind(&item.cidade)
        .bind(&item.estado)
        .bind(item.latitude)
        .bind(item.longitude)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(uuid)
    }

    async fn atualizar(&self, item: EnderecoLoja) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE enderecos_loja SET loja_uuid = $1, cep = $2, logradouro = $3, numero = $4, complemento = $5, bairro = $6, cidade = $7, estado = $8, latitude = $9, longitude = $10
            WHERE uuid = $11
        ")
        .bind(item.loja_uuid)
        .bind(&item.cep)
        .bind(&item.logradouro)
        .bind(&item.numero)
        .bind(&item.complemento)
        .bind(&item.bairro)
        .bind(&item.cidade)
        .bind(&item.estado)
        .bind(item.latitude)
        .bind(item.longitude)
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

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<EnderecoLoja>, String> {
        sqlx::query_as::<_, EnderecoLoja>("SELECT * FROM enderecos_loja WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl EnderecoLojaRepositoryPort for EnderecoLojaRepository {
    async fn criar(&self, endereco: &EnderecoLoja) -> DomainResult<Uuid> {
        <Self as Repository<EnderecoLoja>>::criar(self, endereco).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<EnderecoLoja>> {
        <Self as Repository<EnderecoLoja>>::buscar_por_uuid(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<EnderecoLoja>> {
        sqlx::query_as::<_, EnderecoLoja>("SELECT * FROM enderecos_loja WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await.map_err(|e| DomainError::Internal(e.to_string()))
    }
    async fn atualizar(&self, endereco: EnderecoLoja) -> DomainResult<()> {
        <Self as Repository<EnderecoLoja>>::atualizar(self, endereco).await.map_err(|e| DomainError::Internal(e))
    }
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()> {
        <Self as Repository<EnderecoLoja>>::deletar(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
}
