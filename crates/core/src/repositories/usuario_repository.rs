use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Usuario, Model}, repositories::Repository};
use crate::ports::UsuarioRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct UsuarioRepository { pool: Arc<PgPool> }

#[allow(dead_code)]
impl UsuarioRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_email(&self, email: &str) -> Result<Option<Usuario>, String> {
        sqlx::query_as::<_, Usuario>(
            "SELECT * FROM usuarios WHERE email = $1 AND deletado = false"
        )
        .bind(email)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_username(&self, username: &str) -> Result<Option<Usuario>, String> {
        sqlx::query_as::<_, Usuario>(
            "SELECT * FROM usuarios WHERE username = $1 AND deletado = false"
        )
        .bind(username)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_celular(&self, celular: &str) -> Result<Option<Usuario>, String> {
        sqlx::query_as::<_, Usuario>(
            "SELECT * FROM usuarios WHERE celular = $1 AND deletado = false"
        )
        .bind(celular)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn marcar_primeiro_acesso(&self, uuid: Uuid) -> Result<(), String> {
        sqlx::query(
            "UPDATE usuarios SET passou_pelo_primeiro_acesso = true WHERE uuid = $1 AND deletado = false"
        )
            .bind(uuid)
            .execute(self.pool())
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    // Soft delete methods
    pub async fn marcar_para_remocao(&self, uuid: Uuid) -> Result<(), String> {
        sqlx::query(
            "UPDATE usuarios SET marcado_para_remocao = NOW(), atualizado_em = NOW() WHERE uuid = $1 AND deletado = false"
        )
            .bind(uuid)
            .execute(self.pool())
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn desmarcar_remocao(&self, uuid: Uuid) -> Result<(), String> {
        sqlx::query(
            "UPDATE usuarios SET marcado_para_remocao = NULL, atualizado_em = NOW() WHERE uuid = $1"
        )
            .bind(uuid)
            .execute(self.pool())
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn marcar_como_deletado(&self, uuid: Uuid) -> Result<(), String> {
        sqlx::query(
            "UPDATE usuarios SET deletado = true, atualizado_em = NOW() WHERE uuid = $1"
        )
            .bind(uuid)
            .execute(self.pool())
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn alterar_ativo(&self, uuid: Uuid, ativo: bool) -> Result<(), String> {
        sqlx::query(
            "UPDATE usuarios SET ativo = $2, atualizado_em = NOW() WHERE uuid = $1 AND deletado = false"
        )
            .bind(uuid)
            .bind(ativo)
            .execute(self.pool())
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn toggle_bloqueado(&self, uuid: Uuid) -> Result<bool, String> {
        sqlx::query_scalar(
            "UPDATE usuarios SET bloqueado = NOT bloqueado, atualizado_em = NOW() 
             WHERE uuid = $1 AND deletado = false 
             RETURNING bloqueado"
        )
            .bind(uuid)
            .fetch_one(self.pool())
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn listar_pendentes_remocao(&self) -> Result<Vec<Usuario>, String> {
        sqlx::query_as::<_, Usuario>(
            "SELECT * FROM usuarios WHERE marcado_para_remocao IS NOT NULL AND deletado = false ORDER BY marcado_para_remocao ASC"
        )
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn deletar_pendentes_antigos(&self, limite: chrono::DateTime<chrono::Utc>) -> Result<u64, String> {
        let result = sqlx::query(
            "UPDATE usuarios SET deletado = true, atualizado_em = NOW() WHERE marcado_para_remocao IS NOT NULL AND marcado_para_remocao <= $1 AND deletado = false"
        )
        .bind(limite)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.rows_affected())
    }
}

#[async_trait::async_trait]
impl Repository<Usuario> for UsuarioRepository {
    fn table_name(&self) -> &'static str { "usuarios" }
    fn entity_name(&self) -> &'static str { "Usuário" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &Usuario) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO usuarios (uuid, nome, username, email, senha_hash, celular, classe, modo_de_cadastro, passou_pelo_primeiro_acesso)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ")
        .bind(&item.uuid)
        .bind(&item.nome)
        .bind(&item.username)
        .bind(&item.email)
        .bind(&item.senha_hash)
        .bind(&item.celular)
        .bind(&item.classe)
        .bind(&item.modo_de_cadastro)
        .bind(&item.passou_pelo_primeiro_acesso)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }


    async fn atualizar(&self, item: Usuario) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE usuarios SET username = $1, email = $2, senha_hash = $3, celular = $4, classe = $5, atualizado_em = $6
            WHERE uuid = $7 AND deletado = false
        ")
        .bind(&item.username)
        .bind(&item.email)
        .bind(&item.senha_hash)
        .bind(&item.celular)
        .bind(&item.classe)
        .bind(&item.atualizado_em)
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

    /// Override para excluir soft-deleted users
    async fn listar_todos(&self) -> Result<Vec<Usuario>, String> {
        let query = format!("SELECT * FROM {} WHERE deletado = false ORDER BY criado_em DESC", self.table_name());
        sqlx::query_as::<_, Usuario>(&query)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }

    /// Override para excluir soft-deleted users
    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Usuario>, String> {
        let query = format!("SELECT * FROM {} WHERE uuid = $1 AND deletado = false", self.table_name());
        sqlx::query_as::<_, Usuario>(&query)
            .bind(uuid)
            .fetch_optional(self.pool())
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, _: Uuid) -> Result<Vec<Usuario>, String> {
        Err("não se aplica".into())
    }
}

#[async_trait::async_trait]
impl UsuarioRepositoryPort for UsuarioRepository {
    async fn criar(&self, entity: &Usuario) -> DomainResult<Uuid> {
        <Self as Repository<Usuario>>::criar(self, entity).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Usuario>> {
        <Self as Repository<Usuario>>::buscar_por_uuid(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_email(&self, email: &str) -> DomainResult<Option<Usuario>> {
        self.buscar_por_email(email).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_username(&self, username: &str) -> DomainResult<Option<Usuario>> {
        self.buscar_por_username(username).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_celular(&self, celular: &str) -> DomainResult<Option<Usuario>> {
        self.buscar_por_celular(celular).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_todos(&self) -> DomainResult<Vec<Usuario>> {
        <Self as Repository<Usuario>>::listar_todos(self).await.map_err(|e| DomainError::Internal(e))
    }
    async fn atualizar(&self, entity: Usuario) -> DomainResult<()> {
        <Self as Repository<Usuario>>::atualizar(self, entity).await.map_err(|e| DomainError::Internal(e))
    }
    async fn marcar_primeiro_acesso(&self, uuid: Uuid) -> DomainResult<()> {
        self.marcar_primeiro_acesso(uuid).await.map_err(|e| DomainError::Internal(e))
    }

    // Soft delete port methods
    async fn marcar_para_remocao(&self, uuid: Uuid) -> DomainResult<()> {
        self.marcar_para_remocao(uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn desmarcar_remocao(&self, uuid: Uuid) -> DomainResult<()> {
        self.desmarcar_remocao(uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn marcar_como_deletado(&self, uuid: Uuid) -> DomainResult<()> {
        self.marcar_como_deletado(uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn alterar_ativo(&self, uuid: Uuid, ativo: bool) -> DomainResult<()> {
        self.alterar_ativo(uuid, ativo).await.map_err(|e| DomainError::Internal(e))
    }
    async fn toggle_bloqueado(&self, uuid: Uuid) -> DomainResult<bool> {
        self.toggle_bloqueado(uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_pendentes_remocao(&self) -> DomainResult<Vec<Usuario>> {
        self.listar_pendentes_remocao().await.map_err(|e| DomainError::Internal(e))
    }

    async fn deletar_pendentes_antigos(&self, limite: chrono::DateTime<chrono::Utc>) -> DomainResult<u64> {
        self.deletar_pendentes_antigos(limite).await.map_err(|e| DomainError::Internal(e))
    }
}
