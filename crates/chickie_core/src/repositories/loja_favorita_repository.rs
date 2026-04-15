use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{LojaFavorita, Model}, repositories::Repository};
use crate::ports::LojaFavoritaRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct LojaFavoritaRepository { pool: Arc<PgPool> }

impl LojaFavoritaRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<LojaFavorita>, String> {
        sqlx::query_as::<_, LojaFavorita>("SELECT * FROM lojas_favoritas WHERE usuario_uuid = $1")
        .bind(usuario_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<LojaFavorita>, String> {
        sqlx::query_as::<_, LojaFavorita>("SELECT * FROM lojas_favoritas WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_usuario_e_loja(
        &self,
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
    ) -> Result<Option<LojaFavorita>, String> {
        sqlx::query_as::<_, LojaFavorita>(
            "SELECT * FROM lojas_favoritas WHERE usuario_uuid = $1 AND loja_uuid = $2"
        )
        .bind(usuario_uuid)
        .bind(loja_uuid)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<LojaFavorita> for LojaFavoritaRepository {
    fn table_name(&self) -> &'static str { "lojas_favoritas" }
    fn entity_name(&self) -> &'static str { "Loja favorita" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &LojaFavorita) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO lojas_favoritas (uuid, usuario_uuid, loja_uuid)
            VALUES ($1, $2, $3);
        ")
        .bind(item.uuid)
        .bind(item.usuario_uuid)
        .bind(item.loja_uuid)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: LojaFavorita) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE lojas_favoritas SET usuario_uuid = $1, loja_uuid = $2 WHERE uuid = $3
        ")
        .bind(item.usuario_uuid)
        .bind(item.loja_uuid)
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

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<LojaFavorita>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}

#[async_trait::async_trait]
impl LojaFavoritaRepositoryPort for LojaFavoritaRepository {
    async fn criar(&self, favorita: &LojaFavorita) -> DomainResult<Uuid> {
        <Self as Repository<LojaFavorita>>::criar(self, favorita).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_por_usuario(&self, usuario_uuid: Uuid) -> DomainResult<Vec<LojaFavorita>> {
        self.buscar_por_usuario(usuario_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_usuario_e_loja(&self, usuario_uuid: Uuid, loja_uuid: Uuid) -> DomainResult<Option<LojaFavorita>> {
        self.buscar_por_usuario_e_loja(usuario_uuid, loja_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()> {
        <Self as Repository<LojaFavorita>>::deletar(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
}
