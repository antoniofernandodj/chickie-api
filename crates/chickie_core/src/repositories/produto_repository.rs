use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Produto, Model}, repositories::Repository};
use crate::ports::ProdutoRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct ProdutoRepository { pool: Arc<PgPool> }

#[allow(dead_code)]
impl ProdutoRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("SELECT * FROM produtos WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_categoria(&self, categoria_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("SELECT * FROM produtos WHERE categoria_uuid = $1")
        .bind(categoria_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("SELECT * FROM produtos WHERE loja_uuid = $1 AND disponivel = true")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_nome(&self, nome: &str, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("SELECT * FROM produtos WHERE loja_uuid = $1 AND nome LIKE $2")
        .bind(loja_uuid)
        .bind(format!("%{}%", nome))
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn atualizar_disponibilidade(&self, uuid: Uuid, disponivel: bool) -> Result<(), String> {
        let result = sqlx::query("UPDATE produtos SET disponivel = $1 WHERE uuid = $2")
            .bind(disponivel)
            .bind(uuid)
            .execute(self.pool())
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Produto não encontrado".to_string())
        } else {
            Ok(())
        }
    }
}

#[async_trait::async_trait]
impl Repository<Produto> for ProdutoRepository {
    fn table_name(&self) -> &'static str { "produtos" }
    fn entity_name(&self) -> &'static str { "Produto" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &Produto) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO produtos (uuid, loja_uuid, categoria_uuid, nome, descricao, preco, imagem_url, disponivel, tempo_preparo_min, destaque)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(item.categoria_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.preco)
        .bind(&item.imagem_url)
        .bind(item.disponivel)
        .bind(item.tempo_preparo_min)
        .bind(item.destaque)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Produto) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE produtos SET loja_uuid = $1, categoria_uuid = $2, nome = $3, descricao = $4, preco = $5, imagem_url = $6, disponivel = $7, tempo_preparo_min = $8, destaque = $9
            WHERE uuid = $10
        ")
        .bind(item.loja_uuid)
        .bind(item.categoria_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.preco)
        .bind(&item.imagem_url)
        .bind(item.disponivel)
        .bind(item.tempo_preparo_min)
        .bind(item.destaque)
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

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("SELECT * FROM produtos WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl ProdutoRepositoryPort for ProdutoRepository {
    async fn criar(&self, produto: &Produto) -> DomainResult<Uuid> {
        <Self as Repository<Produto>>::criar(self, produto).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Produto>> {
        <Self as Repository<Produto>>::buscar_por_uuid(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_todos(&self) -> DomainResult<Vec<Produto>> {
        <Self as Repository<Produto>>::listar_todos(self).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_por_categoria(&self, categoria_uuid: Uuid) -> DomainResult<Vec<Produto>> {
        self.buscar_por_categoria(categoria_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Produto>> {
        self.buscar_por_loja(loja_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn atualizar(&self, produto: Produto) -> DomainResult<()> {
        <Self as Repository<Produto>>::atualizar(self, produto).await.map_err(|e| DomainError::Internal(e))
    }
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()> {
        <Self as Repository<Produto>>::deletar(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn atualizar_disponibilidade(&self, uuid: Uuid, disponivel: bool) -> DomainResult<()> {
        self.atualizar_disponibilidade(uuid, disponivel).await.map_err(|e| DomainError::Internal(e))
    }
    async fn atualizar_imagem_url(&self, uuid: Uuid, imagem_url: &str) -> DomainResult<()> {
        sqlx::query("UPDATE produtos SET imagem_url = $1 WHERE uuid = $2")
            .bind(imagem_url).bind(uuid)
            .execute(&*self.pool)
            .await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}
