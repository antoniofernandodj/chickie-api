use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Cliente, Model, Produto}, repositories::Repository};

pub struct ClienteRepository { pool: Arc<PgPool> }

impl ClienteRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Cliente>, String> {
        sqlx::query_as::<_, Cliente>("
            SELECT * FROM clientes WHERE usuario_uuid = $1;
        ")
        .bind(usuario_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Cliente>, String> {
        sqlx::query_as::<_, Cliente>("
            SELECT * FROM clientes WHERE loja_uuid = $1;
        ")
        .bind(loja_uuid)
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

#[async_trait::async_trait]
impl<'a> Repository<Cliente> for ClienteRepository {
    fn table_name(&self) -> String { "clientes".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Cliente>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = $1", t);
        sqlx::query_as::<_, Cliente>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Cliente) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO clientes (
                uuid,
                usuario_uuid,
                loja_uuid,
                criado_em
            )
            VALUES ($1, $2, $3, $4);
        ")
        .bind(item.uuid)
        .bind(item.usuario_uuid)
        .bind(item.loja_uuid)
        .bind(&item.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Cliente) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query(
            "UPDATE clientes
            SET
                usuario_uuid = $1,
                loja_uuid = $2
            WHERE uuid = $3
        ")
        .bind(item.usuario_uuid)
        .bind(item.loja_uuid)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Cliente no encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("
                DELETE FROM clientes WHERE uuid = $1
            ")
            .bind(uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Cliente no encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Cliente>, String> {
        sqlx::query_as::<_, Cliente>("SELECT * FROM clientes;")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Cliente>, String> {
        sqlx::query_as::<_, Cliente>("
                SELECT * FROM clientes
                WHERE loja_uuid = $1;
            ")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}
