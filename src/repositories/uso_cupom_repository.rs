use std::sync::Arc;

use sqlx::postgres::PgPool;
use sqlx::Row;
use uuid::Uuid;
use crate::{models::{UsoCupom, Model}, repositories::Repository};

pub struct UsoCupomRepository { pool: Arc<PgPool> }

impl UsoCupomRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<UsoCupom>, String> {
        sqlx::query_as::<_, UsoCupom>("
            SELECT * FROM uso_cupons
            WHERE usuario_uuid = $1;
        ")
        .bind(usuario_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_cupom(&self, cupom_uuid: Uuid) -> Result<Vec<UsoCupom>, String> {
        sqlx::query_as::<_, UsoCupom>("
            SELECT * FROM uso_cupons
            WHERE cupom_uuid = $1;
        ")
        .bind(cupom_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn contar_usos_usuario(&self, usuario_uuid: Uuid, cupom_uuid: Uuid) -> Result<u32, String> {
        let result = sqlx::query("
            SELECT COUNT(*) as count FROM uso_cupons
            WHERE usuario_uuid = $1 AND cupom_uuid = $2;
        ")
        .bind(usuario_uuid)
        .bind(cupom_uuid)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.try_get::<i64, _>("count").unwrap_or(0) as u32)
    }
}

#[async_trait::async_trait]
impl<'a> Repository<UsoCupom> for UsoCupomRepository {
    fn table_name(&self) -> String { "uso_cupons".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<UsoCupom>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = $1", t);
        sqlx::query_as::<_, UsoCupom>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &UsoCupom) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO uso_cupons (
                uuid,
                cupom_uuid,
                usuario_uuid,
                pedido_uuid,
                usado_em
            )
            VALUES ($1, $2, $3, $4, $5);
        ")
        .bind(item.uuid)
        .bind(item.cupom_uuid)
        .bind(item.usuario_uuid)
        .bind(item.pedido_uuid)
        .bind(&item.usado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: UsoCupom) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE uso_cupons
            SET
                cupom_uuid = $1,
                usuario_uuid = $2,
                pedido_uuid = $3,
                usado_em = $4
            WHERE uuid = $5
        ")
        .bind(item.cupom_uuid)
        .bind(item.usuario_uuid)
        .bind(item.pedido_uuid)
        .bind(item.usado_em)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Uso de cupom no encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
            DELETE FROM uso_cupons
            WHERE uuid = $1
        ")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Uso de cupom no encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<UsoCupom>, String> {
        sqlx::query_as::<_, UsoCupom>("SELECT * FROM uso_cupons")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<UsoCupom>, String> {
        sqlx::query_as::<_, UsoCupom>("
                SELECT * FROM uso_cupons
                WHERE loja_uuid = $1;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}
