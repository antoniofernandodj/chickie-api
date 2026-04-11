use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{CategoriaProdutos, Model}, repositories::Repository};

pub struct CategoriaProdutosRepository { pool: Arc<PgPool> }

#[allow(dead_code)]
impl CategoriaProdutosRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(
        &self,
        loja_uuid: Uuid
    ) -> Result<Vec<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>("SELECT * FROM categorias_produtos WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_nome(&self, nome: &str, loja_uuid: Uuid) -> Result<Option<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>("SELECT * FROM categorias_produtos WHERE loja_uuid = $1 AND nome = $2")
        .bind(loja_uuid)
        .bind(nome)
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
            INSERT INTO categorias_produtos (uuid, loja_uuid, nome, descricao, ordem, pizza_mode)
            VALUES ($1, $2, $3, $4, $5, $6)
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.ordem)
        .bind(item.pizza_mode)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: CategoriaProdutos) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE categorias_produtos SET loja_uuid = $1, nome = $2, descricao = $3, ordem = $4, pizza_mode = $5
            WHERE uuid = $6
        ")
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.ordem)
        .bind(item.pizza_mode)
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
        sqlx::query_as::<_, CategoriaProdutos>("SELECT * FROM categorias_produtos WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }
}
