use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Adicional, Model}, repositories::Repository};

pub struct AdicionalRepository { pool: Arc<PgPool> }

impl AdicionalRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Adicional>, String> {
        sqlx::query_as::<_, Adicional>("SELECT * FROM adicionais WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Adicional>, String> {
        sqlx::query_as::<_, Adicional>(
            "SELECT * FROM adicionais WHERE loja_uuid = $1 AND disponivel = true"
        )
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<Adicional> for AdicionalRepository {
    fn table_name(&self) -> String { "adicionais".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Adicional>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = $1", t);
        sqlx::query_as::<_, Adicional>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Adicional) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO adicionais (uuid, loja_uuid, nome, descricao, preco, disponivel, criado_em)
            VALUES ($1, $2, $3, $4, $5, $6, $7);
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.preco)
        .bind(item.disponivel)
        .bind(&item.criado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Adicional) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE adicionais SET loja_uuid = $1, nome = $2, descricao = $3, preco = $4, disponivel = $5
            WHERE uuid = $6
        ")
        .bind(item.loja_uuid)
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.preco)
        .bind(item.disponivel)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Adicional no encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM adicionais WHERE uuid = $1")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Adicional no encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Adicional>, String> {
        sqlx::query_as::<_, Adicional>("SELECT * FROM adicionais")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Adicional>, String> {
        sqlx::query_as::<_, Adicional>("SELECT * FROM adicionais WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}
