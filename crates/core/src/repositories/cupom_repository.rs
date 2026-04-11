use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Cupom, Model, StatusCupom}, repositories::Repository};
use crate::ports::CupomRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct CupomRepository { pool: Arc<PgPool> }

#[allow(dead_code)]
impl CupomRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_codigo(&self, codigo: &str, loja_uuid: Uuid) -> Result<Option<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("SELECT * FROM cupons WHERE UPPER(codigo) = UPPER($1) and loja_uuid = $2")
        .bind(codigo)
        .bind(loja_uuid)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("SELECT * FROM cupons WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_ativos(&self, loja_uuid: Uuid) -> Result<Vec<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("SELECT * FROM cupons WHERE loja_uuid = $1 AND status = $2")
        .bind(loja_uuid)
        .bind(StatusCupom::Ativo.to_string())
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Cupom> for CupomRepository {
    fn table_name(&self) -> &'static str { "cupons" }
    fn entity_name(&self) -> &'static str { "Cupom" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &Cupom) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO cupons (uuid, loja_uuid, codigo, descricao, tipo_desconto, valor_desconto, valor_minimo, data_validade, limite_uso, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10);
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(&item.codigo)
        .bind(&item.descricao)
        .bind(item.tipo_desconto.to_string())
        .bind(item.valor_desconto)
        .bind(item.valor_minimo)
        .bind(&item.data_validade)
        .bind(item.limite_uso)
        .bind(item.status.to_string())
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Cupom) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE cupons SET loja_uuid = $1, codigo = $2, descricao = $3, tipo_desconto = $4, valor_desconto = $5, valor_minimo = $6, data_validade = $7, limite_uso = $8, status = $9
            WHERE uuid = $10
        ")
        .bind(item.loja_uuid)
        .bind(&item.codigo)
        .bind(&item.descricao)
        .bind(item.tipo_desconto.to_string())
        .bind(item.valor_desconto)
        .bind(item.valor_minimo)
        .bind(item.data_validade)
        .bind(item.limite_uso)
        .bind(item.status.to_string())
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

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("SELECT * FROM cupons WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl CupomRepositoryPort for CupomRepository {
    async fn criar(&self, cupom: &Cupom) -> DomainResult<Uuid> {
        <Self as Repository<Cupom>>::criar(self, cupom).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Cupom>> {
        <Self as Repository<Cupom>>::buscar_por_uuid(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_codigo(&self, codigo: &str, loja_uuid: Uuid) -> DomainResult<Option<Cupom>> {
        self.buscar_por_codigo(codigo, loja_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_ativos(&self, loja_uuid: Uuid) -> DomainResult<Vec<Cupom>> {
        self.buscar_ativos(loja_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_todos(&self) -> DomainResult<Vec<Cupom>> {
        <Self as Repository<Cupom>>::listar_todos(self).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Cupom>> {
        <Self as Repository<Cupom>>::listar_todos_por_loja(self, loja_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn atualizar(&self, cupom: Cupom) -> DomainResult<()> {
        <Self as Repository<Cupom>>::atualizar(self, cupom).await.map_err(|e| DomainError::Internal(e))
    }
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()> {
        <Self as Repository<Cupom>>::deletar(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
}
