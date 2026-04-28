use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{CategoriaProdutos, CategoriaProdutosOrdenada, Model}, repositories::Repository};
use crate::ports::CategoriaRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct CategoriaProdutosRepository { pool: Arc<PgPool> }

#[allow(dead_code)]
impl CategoriaProdutosRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

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
            INSERT INTO categorias_produtos (uuid, loja_uuid, nome, descricao, pizza_mode, drink_mode)
            VALUES ($1, $2, $3, $4, $5, $6)
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
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
            UPDATE categorias_produtos SET loja_uuid = $1, nome = $2, descricao = $3, pizza_mode = $4, drink_mode = $5
            WHERE uuid = $6
        ")
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
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
        sqlx::query_as::<_, CategoriaProdutos>(
            "SELECT * FROM categorias_produtos WHERE loja_uuid IS NULL OR loja_uuid = $1 ORDER BY criado_em ASC"
        )
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl CategoriaRepositoryPort for CategoriaProdutosRepository {
    async fn criar(&self, categoria: &CategoriaProdutos) -> DomainResult<Uuid> {
        <Self as Repository<CategoriaProdutos>>::criar(self, categoria).await.map_err(DomainError::Internal)
    }
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<CategoriaProdutos>> {
        <Self as Repository<CategoriaProdutos>>::buscar_por_uuid(self, uuid).await.map_err(DomainError::Internal)
    }
    async fn listar_todos(&self) -> DomainResult<Vec<CategoriaProdutos>> {
        <Self as Repository<CategoriaProdutos>>::listar_todos(self).await.map_err(DomainError::Internal)
    }
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<CategoriaProdutos>> {
        sqlx::query_as::<_, CategoriaProdutos>(
            "SELECT * FROM categorias_produtos WHERE loja_uuid IS NULL OR loja_uuid = $1 ORDER BY criado_em ASC"
        )
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
    }
    async fn listar_por_loja_com_ordem(&self, loja_uuid: Uuid) -> DomainResult<Vec<CategoriaProdutosOrdenada>> {
        // Categorias com ordem explícita usam o valor da tabela ordem_categorias_de_produtos.
        // Categorias sem ordem recebem fallback = MAX(ordem_da_loja) + posição_por_criado_em,
        // garantindo que sempre retornamos i32 e que aparecem após as ordenadas explicitamente.
        sqlx::query_as::<_, CategoriaProdutosOrdenada>("
            WITH max_ordem AS (
                SELECT COALESCE(MAX(ordem), 0) AS max_val
                FROM ordem_categorias_de_produtos
                WHERE loja_uuid = $1
            ),
            sem_ordem AS (
                SELECT cp.uuid,
                       ROW_NUMBER() OVER (ORDER BY cp.criado_em ASC) AS pos
                FROM categorias_produtos cp
                LEFT JOIN ordem_categorias_de_produtos ocp
                    ON ocp.categoria_uuid = cp.uuid AND ocp.loja_uuid = $1
                WHERE (cp.loja_uuid IS NULL OR cp.loja_uuid = $1)
                  AND ocp.ordem IS NULL
            )
            SELECT cp.uuid, cp.loja_uuid, cp.nome, cp.descricao, cp.pizza_mode, cp.drink_mode, cp.criado_em,
                   COALESCE(ocp.ordem, ((SELECT max_val FROM max_ordem) + so.pos)::integer) AS ordem
            FROM categorias_produtos cp
            LEFT JOIN ordem_categorias_de_produtos ocp
                ON ocp.categoria_uuid = cp.uuid AND ocp.loja_uuid = $1
            LEFT JOIN sem_ordem so ON so.uuid = cp.uuid
            WHERE cp.loja_uuid IS NULL OR cp.loja_uuid = $1
            ORDER BY ocp.ordem ASC NULLS LAST, cp.criado_em ASC
        ")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
    }
    async fn listar_globais(&self) -> DomainResult<Vec<CategoriaProdutos>> {
        sqlx::query_as::<_, CategoriaProdutos>(
            "SELECT * FROM categorias_produtos WHERE loja_uuid IS NULL ORDER BY criado_em ASC"
        )
        .fetch_all(self.pool())
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
    }
    async fn atualizar(&self, categoria: CategoriaProdutos) -> DomainResult<()> {
        <Self as Repository<CategoriaProdutos>>::atualizar(self, categoria).await.map_err(DomainError::Internal)
    }
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()> {
        <Self as Repository<CategoriaProdutos>>::deletar(self, uuid).await.map_err(DomainError::Internal)
    }
    async fn contar_produtos(&self, categoria_uuid: Uuid) -> DomainResult<i64> {
        sqlx::query_scalar("SELECT COUNT(*) FROM produtos WHERE categoria_uuid = $1")
            .bind(categoria_uuid)
            .fetch_one(&*self.pool)
            .await.map_err(|e| DomainError::Internal(e.to_string()))
    }
}
