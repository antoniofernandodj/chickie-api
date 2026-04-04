use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Ingrediente, Model}, repositories::Repository};

pub struct IngredienteRepository { pool: Arc<PgPool> }

impl IngredienteRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Ingrediente>, String> {
        sqlx::query_as::<_, Ingrediente>("SELECT * FROM ingredientes WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Ingrediente>, String> {
        sqlx::query_as::<_, Ingrediente>("SELECT * FROM ingredientes WHERE loja_uuid = $1 AND quantidade > 0")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Ingrediente> for IngredienteRepository {
    fn table_name(&self) -> &'static str { "ingredientes" }
    fn entity_name(&self) -> &'static str { "Ingrediente" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &Ingrediente) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO ingredientes (uuid, loja_uuid, nome, unidade_medida, quantidade, preco_unitario, criado_em, atualizado_em)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.unidade_medida)
        .bind(item.quantidade)
        .bind(item.preco_unitario)
        .bind(&item.criado_em)
        .bind(&item.atualizado_em)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Ingrediente) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE ingredientes SET loja_uuid = $1, nome = $2, unidade_medida = $3, quantidade = $4, preco_unitario = $5, atualizado_em = $6
            WHERE uuid = $7
        ")
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.unidade_medida)
        .bind(item.quantidade)
        .bind(item.preco_unitario)
        .bind(item.atualizado_em)
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

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Ingrediente>, String> {
        sqlx::query_as::<_, Ingrediente>("SELECT * FROM ingredientes WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }
}
