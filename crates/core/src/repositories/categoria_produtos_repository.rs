use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{CategoriaProdutos, Model}, repositories::Repository};
use crate::ports::CategoriaRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};
use sqlx::Acquire;

pub struct CategoriaProdutosRepository { pool: Arc<PgPool> }

#[allow(dead_code)]
impl CategoriaProdutosRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(
        &self,
        loja_uuid: Uuid
    ) -> Result<Vec<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>("SELECT * FROM categorias_produtos WHERE loja_uuid IS NULL OR loja_uuid = $1 ORDER BY COALESCE(loja_uuid::text, '0'), ordem")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_nome(&self, nome: &str, loja_uuid: Option<Uuid>) -> Result<Option<CategoriaProdutos>, String> {
        let query = if let Some(uuid) = loja_uuid {
            sqlx::query_as::<_, CategoriaProdutos>("SELECT * FROM categorias_produtos WHERE loja_uuid = $1 AND nome = $2")
                .bind(uuid)
                .bind(nome)
        } else {
            sqlx::query_as::<_, CategoriaProdutos>("SELECT * FROM categorias_produtos WHERE loja_uuid IS NULL AND nome = $1")
                .bind(nome)
        };

        query
        .fetch_optional(self.pool())
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<CategoriaProdutos> for CategoriaProdutosRepository {
    fn table_name(&self) -> &'static str { "categorias_produtos" }
    fn entity_name(&self) -> &'static str { "Categoria" }
    fn entity_gender_suffix(&self) -> &'static str { "a" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &CategoriaProdutos) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO categorias_produtos (uuid, loja_uuid, nome, descricao, ordem, pizza_mode, drink_mode)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.ordem)
        .bind(item.pizza_mode)
        .bind(item.drink_mode)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: CategoriaProdutos) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE categorias_produtos SET loja_uuid = $1, nome = $2, descricao = $3, ordem = $4, pizza_mode = $5, drink_mode = $6
            WHERE uuid = $7
        ")
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.ordem)
        .bind(item.pizza_mode)
        .bind(item.drink_mode)
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

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>("SELECT * FROM categorias_produtos WHERE loja_uuid IS NULL OR loja_uuid = $1 ORDER BY COALESCE(loja_uuid::text, '0'), ordem")
            .bind(loja_uuid)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl CategoriaRepositoryPort for CategoriaProdutosRepository {
    async fn criar(&self, categoria: &CategoriaProdutos) -> DomainResult<Uuid> {
        <Self as Repository<CategoriaProdutos>>::criar(self, categoria).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<CategoriaProdutos>> {
        <Self as Repository<CategoriaProdutos>>::buscar_por_uuid(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_todos(&self) -> DomainResult<Vec<CategoriaProdutos>> {
        <Self as Repository<CategoriaProdutos>>::listar_todos(self).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<CategoriaProdutos>> {
        self.buscar_por_loja(loja_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_globais(&self) -> DomainResult<Vec<CategoriaProdutos>> {
        sqlx::query_as::<_, CategoriaProdutos>("SELECT * FROM categorias_produtos WHERE loja_uuid IS NULL ORDER BY ordem")
            .fetch_all(self.pool())
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))
    }
    async fn atualizar(&self, categoria: CategoriaProdutos) -> DomainResult<()> {
        <Self as Repository<CategoriaProdutos>>::atualizar(self, categoria).await.map_err(|e| DomainError::Internal(e))
    }
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()> {
        <Self as Repository<CategoriaProdutos>>::deletar(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn contar_produtos(&self, categoria_uuid: Uuid) -> DomainResult<i64> {
        sqlx::query_scalar("SELECT COUNT(*) FROM produtos WHERE categoria_uuid = $1")
            .bind(categoria_uuid)
            .fetch_one(&*self.pool)
            .await.map_err(|e| DomainError::Internal(e.to_string()))
    }

    async fn proxima_ordem(&self, loja_uuid: Option<Uuid>) -> DomainResult<i32> {
        let query = if let Some(uuid) = loja_uuid {
            sqlx::query_scalar("SELECT MAX(ordem) FROM categorias_produtos WHERE loja_uuid = $1")
                .bind(uuid)
        } else {
            sqlx::query_scalar("SELECT MAX(ordem) FROM categorias_produtos WHERE loja_uuid IS NULL")
        };

        let max: Option<i32> = query
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(max.unwrap_or(0) + 1)
    }

    async fn reordenar(&self, loja_uuid: Option<Uuid>, reordenacoes: Vec<(Uuid, i32)>) -> DomainResult<()> {
        let mut conn = self.pool.acquire().await.map_err(|e| DomainError::Internal(e.to_string()))?;
        let mut tx = conn.begin().await.map_err(|e| DomainError::Internal(e.to_string()))?;

        for (uuid, ordem) in reordenacoes {
            let query = if let Some(l_uuid) = loja_uuid {
                sqlx::query("UPDATE categorias_produtos SET ordem = $1 WHERE uuid = $2 AND loja_uuid = $3")
                    .bind(ordem).bind(uuid).bind(l_uuid)
            } else {
                sqlx::query("UPDATE categorias_produtos SET ordem = $1 WHERE uuid = $2 AND loja_uuid IS NULL")
                    .bind(ordem).bind(uuid)
            };

            query
            .execute(&mut *tx)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        }

        tx.commit().await.map_err(|e| DomainError::Internal(e.to_string()))
    }
}
