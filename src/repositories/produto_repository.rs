use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Produto, Model}, repositories::Repository};

pub struct ProdutoRepository { pool: Arc<PgPool> }

impl ProdutoRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("
            SELECT * FROM produtos
            WHERE loja_uuid = $1;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_categoria(&self, categoria_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("
            SELECT * FROM produtos
            WHERE categoria_uuid = $1;
        ")
        .bind(categoria_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("
            SELECT * FROM produtos
            WHERE loja_uuid = $1 AND disponivel = true;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_nome(&self, nome: &str, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("
            SELECT * FROM produtos
            WHERE loja_uuid = $1 AND nome LIKE $2;
        ")
        .bind(loja_uuid)
        .bind(format!("%{}%", nome))
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<Produto> for ProdutoRepository {
    fn table_name(&self) -> String { "produtos".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Produto>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = $1", t);
        sqlx::query_as::<_, Produto>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Produto) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO produtos (
                uuid,
                loja_uuid,
                categoria_uuid,
                nome,
                descricao,
                preco,
                imagem_url,
                disponivel,
                tempo_preparo_min,
                destaque,
                criado_em,
                atualizado_em
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);
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
        .bind(&item.criado_em)
        .bind(&item.atualizado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Produto) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE produtos
            SET
                loja_uuid = $1,
                categoria_uuid = $2,
                nome = $3,
                descricao = $4,
                preco = $5,
                imagem_url = $6,
                disponivel = $7,
                tempo_preparo_min = $8,
                destaque = $9,
                atualizado_em = $10,
            WHERE uuid = $11
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
        .bind(item.atualizado_em)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Produto no encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
                DELETE FROM produtos WHERE uuid = $1
            ")
            .bind(uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Produto no encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("SELECT * FROM produtos")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("
                SELECT * FROM produtos
                WHERE loja_uuid = $1;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}
