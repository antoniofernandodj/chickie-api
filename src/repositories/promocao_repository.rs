use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Promocao, Model, StatusCupom}, repositories::Repository};

pub struct PromocaoRepository { pool: Arc<PgPool> }

impl PromocaoRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Promocao>, String> {
        sqlx::query_as::<_, Promocao>("
            SELECT * FROM promocoes
            WHERE loja_uuid = $1;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_ativas(&self, loja_uuid: Uuid) -> Result<Vec<Promocao>, String> {
        sqlx::query_as::<_, Promocao>("
            SELECT * FROM promocoes
            WHERE loja_uuid = $1 AND status = $2;
        ")
        .bind(loja_uuid)
        .bind(StatusCupom::Ativo.to_string())
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_prioridade(&self, loja_uuid: Uuid) -> Result<Vec<Promocao>, String> {
        sqlx::query_as::<_, Promocao>("
            SELECT * FROM promocoes
            WHERE loja_uuid = $1 AND status = $2
            ORDER BY prioridade DESC;
        ")
        .bind(loja_uuid)
        .bind(StatusCupom::Ativo.to_string())
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<Promocao> for PromocaoRepository {
    fn table_name(&self) -> String { "promocoes".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Promocao>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = $1", t);
        sqlx::query_as::<_, Promocao>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Promocao) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO promocoes (
                uuid,
                loja_uuid,
                nome,
                descricao,
                tipo_desconto,
                valor_desconto,
                data_inicio,
                data_fim,
                prioridade,
                status,
                criado_em
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);
        ")
        .bind(&item.uuid)
        .bind(&item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(&item.tipo_desconto)
        .bind(&item.valor_desconto)
        .bind(&item.data_inicio)
        .bind(&item.data_fim)
        .bind(&item.prioridade)
        .bind(item.status.to_string())
        .bind(&item.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Promocao) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE promocoes
            SET
                loja_uuid = $1,
                nome = $2,
                descricao = $3,
                tipo_desconto = $4,
                valor_desconto = $5,
                data_inicio = $6,
                data_fim = $7,
                prioridade = $8,
                status = $9
             WHERE uuid = $10
        ")
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.tipo_desconto.to_string())
        .bind(item.valor_desconto)
        .bind(item.data_inicio)
        .bind(item.data_fim)
        .bind(item.prioridade)
        .bind(item.status.to_string())
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Promocao no encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM promocoes
            WHERE uuid = $1
        ")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Promocao no encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Promocao>, String> {
        sqlx::query_as::<_, Promocao>("SELECT * FROM promocoes")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Promocao>, String> {
        sqlx::query_as::<_, Promocao>("
                SELECT * FROM promocoes
                WHERE loja_uuid = $1;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}
