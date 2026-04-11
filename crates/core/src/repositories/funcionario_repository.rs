use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Funcionario, Model}, repositories::Repository};

pub struct FuncionarioRepository { pool: Arc<PgPool> }

#[allow(dead_code)]
impl FuncionarioRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Funcionario>, String> {
        sqlx::query_as::<_, Funcionario>("SELECT * FROM funcionarios WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_cargo(&self, cargo: &str, loja_uuid: Uuid) -> Result<Vec<Funcionario>, String> {
        sqlx::query_as::<_, Funcionario>("SELECT * FROM funcionarios WHERE loja_uuid = $1 AND cargo = $2")
        .bind(loja_uuid)
        .bind(cargo)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Option<Funcionario>, String> {
        sqlx::query_as::<_, Funcionario>("SELECT * FROM funcionarios WHERE usuario_uuid = $1")
        .bind(usuario_uuid)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Funcionario> for FuncionarioRepository {
    fn table_name(&self) -> &'static str { "funcionarios" }
    fn entity_name(&self) -> &'static str { "Funcionário" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &Funcionario) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO funcionarios (uuid, loja_uuid, usuario_uuid, cargo, salario, data_admissao)
            VALUES ($1, $2, $3, $4, $5, $6)
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(item.usuario_uuid)
        .bind(&item.cargo)
        .bind(item.salario)
        .bind(item.data_admissao)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: Funcionario) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE funcionarios SET loja_uuid = $1, usuario_uuid = $2, cargo = $3, salario = $4, data_admissao = $5
            WHERE uuid = $6
        ")
        .bind(item.loja_uuid)
        .bind(item.usuario_uuid)
        .bind(&item.cargo)
        .bind(item.salario)
        .bind(item.data_admissao)
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

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Funcionario>, String> {
        sqlx::query_as::<_, Funcionario>("SELECT * FROM funcionarios WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }
}
