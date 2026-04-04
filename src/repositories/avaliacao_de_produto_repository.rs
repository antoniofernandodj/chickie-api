use std::sync::Arc;

use sqlx::postgres::PgPool;
use sqlx::Row;
use uuid::Uuid;
use crate::{models::{AvaliacaoDeProduto, Model}, repositories::Repository};

pub struct AvaliacaoDeProdutoRepository { pool: Arc<PgPool> }

impl AvaliacaoDeProdutoRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_produto(&self, produto_uuid: Uuid) -> Result<Vec<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>("SELECT * FROM avaliacoes_produto WHERE produto_uuid = $1")
        .bind(produto_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>("SELECT * FROM avaliacoes_produto WHERE usuario_uuid = $1")
        .bind(usuario_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_pedido(&self, pedido_uuid: Uuid) -> Result<Vec<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>("SELECT * FROM avaliacoes_produto WHERE pedido_uuid = $1")
        .bind(pedido_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn calcular_media(&self, produto_uuid: Uuid) -> Result<f64, String> {
        let result = sqlx::query("SELECT AVG(nota) as media FROM avaliacoes_produto WHERE produto_uuid = $1")
        .bind(produto_uuid)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.try_get("media").unwrap_or(0.0))
    }
}

#[async_trait::async_trait]
impl<'a> Repository<AvaliacaoDeProduto> for AvaliacaoDeProdutoRepository {
    fn table_name(&self) -> String { "avaliacoes_produto".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<AvaliacaoDeProduto>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = $1", t);
        sqlx::query_as::<_, AvaliacaoDeProduto>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &AvaliacaoDeProduto) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO avaliacoes_produto (uuid, usuario_uuid, loja_uuid, produto_uuid, nota, descricao, comentario, criado_em)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8);
        ")
        .bind(item.uuid)
        .bind(item.usuario_uuid)
        .bind(item.loja_uuid)
        .bind(item.produto_uuid)
        .bind(item.nota)
        .bind(item.descricao.clone())
        .bind(&item.comentario)
        .bind(&item.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: AvaliacaoDeProduto) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE avaliacoes_produto SET produto_uuid = $1, usuario_uuid = $2, nota = $3, comentario = $4
            WHERE uuid = $5
        ")
        .bind(item.usuario_uuid)
        .bind(item.produto_uuid)
        .bind(item.nota)
        .bind(&item.comentario)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Avaliacao no encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM avaliacoes_produto WHERE uuid = $1")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Avaliacao no encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>("SELECT * FROM avaliacoes_produto")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>("SELECT * FROM avaliacoes_produto WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}
