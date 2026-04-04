use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Usuario, Model}, repositories::Repository};

pub struct UsuarioRepository { pool: Arc<PgPool> }

impl UsuarioRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_email(&self, email: &str) -> Result<Option<Usuario>, String> {
        sqlx::query_as::<_, Usuario>("SELECT * FROM usuarios WHERE email = $1")
        .bind(email)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_username(&self, username: &str) -> Result<Option<Usuario>, String> {
        sqlx::query_as::<_, Usuario>("SELECT * FROM usuarios WHERE username = $1")
        .bind(username)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_telefone(&self, telefone: &str) -> Result<Option<Usuario>, String> {
        sqlx::query_as::<_, Usuario>("SELECT * FROM usuarios WHERE telefone = $1")
        .bind(telefone)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<Usuario> for UsuarioRepository {
    fn table_name(&self) -> String { "usuarios".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Usuario>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = $1", t);
        sqlx::query_as::<_, Usuario>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn criar(&self, item: &Usuario) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO usuarios (uuid, nome, username, email, senha_hash, telefone, celular, criado_em, atualizado_em)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);
        ")
        .bind(&item.uuid)
        .bind(&item.nome)
        .bind(&item.username)
        .bind(&item.email)
        .bind(&item.senha_hash)
        .bind(&item.telefone)
        .bind(&item.celular)
        .bind(&item.criado_em)
        .bind(&item.atualizado_em)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }


    async fn atualizar(&self, item: Usuario) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE usuarios SET username = $1, email = $2, senha_hash = $3, telefone = $4, atualizado_em = $5
            WHERE uuid = $6
        ")
        .bind(&item.username)
        .bind(&item.email)
        .bind(&item.senha_hash)
        .bind(&item.telefone)
        .bind("")
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Usuário não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM usuarios WHERE uuid = $1")
            .bind(uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Usuário não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Usuario>, String> {
        sqlx::query_as::<_, Usuario>("SELECT * FROM usuarios;")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, _: Uuid) -> Result<Vec<Usuario>, String> {
        Err("não se aplica".into())
    }
}
