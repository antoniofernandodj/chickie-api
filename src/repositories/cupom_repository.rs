use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Cupom, Model, StatusCupom}, repositories::Repository};

pub struct CupomRepository { pool: Arc<PgPool> }

impl CupomRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_codigo(&self, codigo: &str) -> Result<Option<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("SELECT * FROM cupons WHERE UPPER(codigo) = UPPER($1)")
        .bind(codigo)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("SELECT * FROM cupons WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_ativos(&self, loja_uuid: Uuid) -> Result<Vec<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("SELECT * FROM cupons WHERE loja_uuid = $1 AND status = $2")
        .bind(loja_uuid)
        .bind(StatusCupom::Ativo.to_string())
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<Cupom> for CupomRepository {
    fn table_name(&self) -> String { "cupons".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Cupom>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = $1", t);
        sqlx::query_as::<_, Cupom>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Cupom) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO cupons (uuid, loja_uuid, codigo, descricao, tipo_desconto, valor_desconto, valor_minimo, data_validade, limite_uso, status, criado_em)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);
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
        .bind(&item.criado_em)
        .execute(&*self.pool)
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
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Cupom no encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM cupons WHERE uuid = $1")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Cupom no encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("SELECT * FROM cupons")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("SELECT * FROM cupons WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}
