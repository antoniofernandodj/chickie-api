use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{CategoriaProdutos, Model}, repositories::Repository};

pub struct CategoriaProdutosRepository { pool: Arc<PgPool> }

impl CategoriaProdutosRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(
        &self,
        loja_uuid: Uuid
    ) -> Result<Vec<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>("
            SELECT * FROM categorias_produtos
            WHERE loja_uuid = $1
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_nome(&self, nome: &str, loja_uuid: Uuid) -> Result<Option<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>("
            SELECT * FROM categorias_produtos
            WHERE loja_uuid = $1 AND nome = $2
        ")
        .bind(loja_uuid)
        .bind(nome)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<CategoriaProdutos> for CategoriaProdutosRepository {
    fn table_name(&self) -> String { "categorias_produtos".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<CategoriaProdutos>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = $1", t);
        sqlx::query_as::<_, CategoriaProdutos>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &CategoriaProdutos) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO categorias_produtos (
                uuid,
                loja_uuid,
                nome,
                descricao,
                ordem,
                criado_em
            )
            VALUES ($1, $2, $3, $4, $5, $6)
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.ordem)
        .bind(&item.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: CategoriaProdutos) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE categorias_produtos
                SET loja_uuid = $1,
                nome = $2,
                descricao = $3,
                ordem = $4
             WHERE uuid = $5
        ")
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.ordem)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Categoria no encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
                DELETE FROM categorias_produtos WHERE uuid = $1
            ")
            .bind(uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Categoria no encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>("SELECT * FROM categorias_produtos")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>("
                SELECT * FROM categorias_produtos
                WHERE loja_uuid = $1;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}
