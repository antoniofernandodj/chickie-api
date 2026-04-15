use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{Loja, Model}, repositories::Repository};
use crate::ports::LojaRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct LojaRepository { pool: Arc<PgPool> }

#[allow(dead_code)]
impl LojaRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_email(&self, email: &str) -> Result<Option<Loja>, String> {
        sqlx::query_as::<_, Loja>(
            "SELECT * FROM lojas WHERE email = $1 AND deletado = false"
        )
        .bind(email)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_slug(&self, slug: &str) -> Result<Option<Loja>, String> {
        sqlx::query_as::<_, Loja>(
            "SELECT * FROM lojas WHERE slug = $1 AND deletado = false"
        )
        .bind(slug)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn listar_ativas(&self) -> Result<Vec<Loja>, String> {
        sqlx::query_as::<_, Loja>(
            "SELECT * FROM lojas WHERE ativa = true AND deletado = false AND marcado_para_remocao IS NULL AND ativo = true"
        )
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_criador(&self, admin_uuid: Uuid) -> Result<Vec<Loja>, String> {
        sqlx::query_as::<_, Loja>(
            "SELECT * FROM lojas WHERE criado_por = $1 AND deletado = false ORDER BY criado_em DESC"
        )
        .bind(admin_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn pesquisar(&self, termo: &str) -> Result<Vec<Loja>, String> {
        let pattern = format!("%{}%", termo);
        sqlx::query_as::<_, Loja>("
            SELECT * FROM lojas
            WHERE (nome ILIKE $1
               OR slug ILIKE $1
               OR descricao ILIKE $1
               OR email ILIKE $1)
               AND deletado = false
               AND marcado_para_remocao IS NULL
               AND ativo = true
            ORDER BY nome ASC
        ")
        .bind(pattern)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    // Soft delete methods
    pub async fn marcar_para_remocao(&self, uuid: Uuid) -> Result<(), String> {
        sqlx::query(
            "UPDATE lojas SET marcado_para_remocao = NOW(), atualizado_em = NOW() WHERE uuid = $1 AND deletado = false"
        )
            .bind(uuid)
            .execute(self.pool())
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn desmarcar_remocao(&self, uuid: Uuid) -> Result<(), String> {
        sqlx::query(
            "UPDATE lojas SET marcado_para_remocao = NULL, atualizado_em = NOW() WHERE uuid = $1"
        )
            .bind(uuid)
            .execute(self.pool())
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn marcar_como_deletado(&self, uuid: Uuid) -> Result<(), String> {
        sqlx::query(
            "UPDATE lojas SET deletado = true, atualizado_em = NOW() WHERE uuid = $1"
        )
            .bind(uuid)
            .execute(self.pool())
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn alterar_ativo(&self, uuid: Uuid, ativo: bool) -> Result<(), String> {
        sqlx::query(
            "UPDATE lojas SET ativo = $2, atualizado_em = NOW() WHERE uuid = $1 AND deletado = false"
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
            "UPDATE lojas SET bloqueado = NOT bloqueado, atualizado_em = NOW() 
             WHERE uuid = $1 AND deletado = false 
             RETURNING bloqueado"
        )
            .bind(uuid)
            .fetch_one(self.pool())
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn listar_pendentes_remocao(&self) -> Result<Vec<Loja>, String> {
        sqlx::query_as::<_, Loja>(
            "SELECT * FROM lojas WHERE marcado_para_remocao IS NOT NULL AND deletado = false ORDER BY marcado_para_remocao ASC"
        )
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn deletar_pendentes_antigas(&self, limite: chrono::DateTime<chrono::Utc>) -> Result<u64, String> {
        let result = sqlx::query(
            "UPDATE lojas SET deletado = true, atualizado_em = NOW() WHERE marcado_para_remocao IS NOT NULL AND marcado_para_remocao <= $1 AND deletado = false"
        )
        .bind(limite)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.rows_affected())
    }
}

#[async_trait::async_trait]
impl Repository<Loja> for LojaRepository {
    fn table_name(&self) -> &'static str { "lojas" }
    fn entity_name(&self) -> &'static str { "Loja" }
    fn entity_gender_suffix(&self) -> &'static str { "a" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &Loja) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO lojas (uuid, nome, slug, descricao, email, celular, ativa, logo_url, banner_url, horario_abertura, horario_fechamento, dias_funcionamento, tempo_preparo_min, taxa_entrega, valor_minimo_pedido, raio_entrega_km, criado_por)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        ")
        .bind(item.uuid)
        .bind(&item.nome)
        .bind(&item.slug)
        .bind(&item.descricao)
        .bind(&item.email)
        .bind(&item.celular)
        .bind(item.ativa)
        .bind(&item.logo_url)
        .bind(&item.banner_url)
        .bind(&item.horario_abertura)
        .bind(&item.horario_fechamento)
        .bind(&item.dias_funcionamento)
        .bind(item.tempo_preparo_min)
        .bind(item.taxa_entrega)
        .bind(item.valor_minimo_pedido)
        .bind(item.raio_entrega_km)
        .bind(&item.criado_por)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }



    async fn atualizar(&self, item: Loja) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE lojas SET nome = $1, slug = $2, descricao = $3, email = $4, celular = $5, ativa = $6, logo_url = $7, banner_url = $8, horario_abertura = $9, horario_fechamento = $10, dias_funcionamento = $11, tempo_preparo_min = $12, taxa_entrega = $13, valor_minimo_pedido = $14, raio_entrega_km = $15
            WHERE uuid = $16 AND deletado = false
        ")
        .bind(&item.nome)
        .bind(&item.slug)
        .bind(&item.descricao)
        .bind(&item.email)
        .bind(&item.celular)
        .bind(item.ativa)
        .bind(&item.logo_url)
        .bind(&item.banner_url)
        .bind(&item.horario_abertura)
        .bind(&item.horario_fechamento)
        .bind(&item.dias_funcionamento)
        .bind(item.tempo_preparo_min)
        .bind(item.taxa_entrega)
        .bind(item.valor_minimo_pedido)
        .bind(item.raio_entrega_km)
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

    /// Override para excluir soft-deleted lojas
    async fn listar_todos(&self) -> Result<Vec<Loja>, String> {
        let query = format!("SELECT * FROM {} WHERE deletado = false ORDER BY criado_em DESC", self.table_name());
        sqlx::query_as::<_, Loja>(&query)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }

    /// Override para excluir soft-deleted lojas
    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<Loja>, String> {
        let query = format!("SELECT * FROM {} WHERE uuid = $1 AND deletado = false", self.table_name());
        sqlx::query_as::<_, Loja>(&query)
            .bind(uuid)
            .fetch_optional(self.pool())
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, _: Uuid) -> Result<Vec<Loja>, String> {
        Err("não se aplica".into())
    }
}

#[async_trait::async_trait]
impl LojaRepositoryPort for LojaRepository {
    async fn criar(&self, entity: &Loja) -> DomainResult<Uuid> {
        <Self as Repository<Loja>>::criar(self, entity).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Loja>> {
        <Self as Repository<Loja>>::buscar_por_uuid(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_slug(&self, slug: &str) -> DomainResult<Option<Loja>> {
        self.buscar_por_slug(slug).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_todos(&self) -> DomainResult<Vec<Loja>> {
        <Self as Repository<Loja>>::listar_todos(self).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_criador(&self, criador_uuid: Uuid) -> DomainResult<Vec<Loja>> {
        self.buscar_por_criador(criador_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn pesquisar(&self, termo: &str) -> DomainResult<Vec<Loja>> {
        self.pesquisar(termo).await.map_err(|e| DomainError::Internal(e))
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
    async fn listar_pendentes_remocao(&self) -> DomainResult<Vec<Loja>> {
        self.listar_pendentes_remocao().await.map_err(|e| DomainError::Internal(e))
    }

    async fn deletar_pendentes_antigas(&self, limite: chrono::DateTime<chrono::Utc>) -> DomainResult<u64> {
        self.deletar_pendentes_antigas(limite).await.map_err(|e| DomainError::Internal(e))
    }
}
