use std::sync::Arc;

use sqlx::postgres::PgPool;
use sqlx::Row;
use uuid::Uuid;
use crate::{models::{AvaliacaoDeLoja, AvaliacaoDeLojaComUsuario, Model}, repositories::Repository};
use crate::ports::AvaliacaoDeLojaRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct AvaliacaoDeLojaRepository { pool: Arc<PgPool> }

#[allow(dead_code)]
impl AvaliacaoDeLojaRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<AvaliacaoDeLoja>, String> {
        sqlx::query_as::<_, AvaliacaoDeLoja>("SELECT * FROM avaliacoes_loja WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<AvaliacaoDeLoja>, String> {
        sqlx::query_as::<_, AvaliacaoDeLoja>("SELECT * FROM avaliacoes_loja WHERE usuario_uuid = $1")
        .bind(usuario_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn calcular_media(&self, loja_uuid: Uuid) -> Result<f64, String> {
        let result = sqlx::query("SELECT AVG(nota) as media FROM avaliacoes_loja WHERE loja_uuid = $1")
        .bind(loja_uuid)
        .fetch_one(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.try_get("media").unwrap_or(0.0))
    }

}

#[async_trait::async_trait]
impl Repository<AvaliacaoDeLoja> for AvaliacaoDeLojaRepository {
    fn table_name(&self) -> &'static str { "avaliacoes_loja" }
    fn entity_name(&self) -> &'static str { "Avaliação" }
    fn entity_gender_suffix(&self) -> &'static str { "a" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &AvaliacaoDeLoja) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO avaliacoes_loja (uuid, loja_uuid, usuario_uuid, nota, comentario)
            VALUES ($1, $2, $3, $4, $5);
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(item.usuario_uuid)
        .bind(item.nota)
        .bind(&item.comentario)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(item.uuid)
    }

    async fn atualizar(&self, item: AvaliacaoDeLoja) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE avaliacoes_loja SET loja_uuid = $1, usuario_uuid = $2, nota = $3, comentario = $4
            WHERE uuid = $5
        ")
        .bind(item.loja_uuid)
        .bind(item.usuario_uuid)
        .bind(item.nota)
        .bind(&item.comentario)
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

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<AvaliacaoDeLoja>, String> {
        sqlx::query_as::<_, AvaliacaoDeLoja>("SELECT * FROM avaliacoes_loja WHERE loja_uuid = $1")
            .bind(loja_uuid)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl AvaliacaoDeLojaRepositoryPort for AvaliacaoDeLojaRepository {
    async fn criar(&self, avaliacao: &AvaliacaoDeLoja) -> DomainResult<Uuid> {
        <Self as Repository<AvaliacaoDeLoja>>::criar(self, avaliacao).await.map_err(|e| DomainError::Internal(e))
    }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<AvaliacaoDeLoja>> {
        sqlx::query_as::<_, AvaliacaoDeLoja>("SELECT * FROM avaliacoes_loja WHERE uuid = $1")
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))
    }

    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<AvaliacaoDeLoja>> {
        sqlx::query_as::<_, AvaliacaoDeLoja>("SELECT * FROM avaliacoes_loja WHERE loja_uuid = $1 ORDER BY criado_em DESC")
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))
    }

    async fn listar_por_loja_com_usuario(&self, loja_uuid: Uuid) -> DomainResult<Vec<AvaliacaoDeLojaComUsuario>> {
        sqlx::query_as::<_, AvaliacaoDeLojaComUsuario>(
            "SELECT a.uuid, a.loja_uuid, a.usuario_uuid,
                    u.nome AS usuario_nome, u.email AS usuario_email,
                    a.nota, a.comentario, a.criado_em
             FROM avaliacoes_loja a
             JOIN usuarios u ON a.usuario_uuid = u.uuid
             WHERE a.loja_uuid = $1
             ORDER BY a.criado_em DESC"
        )
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))
    }

    async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> DomainResult<Vec<AvaliacaoDeLoja>> {
        sqlx::query_as::<_, AvaliacaoDeLoja>("SELECT * FROM avaliacoes_loja WHERE usuario_uuid = $1 ORDER BY criado_em DESC")
            .bind(usuario_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))
    }

    async fn buscar_por_usuario_e_loja(&self, usuario_uuid: Uuid, loja_uuid: Uuid) -> DomainResult<Option<AvaliacaoDeLoja>> {
        sqlx::query_as::<_, AvaliacaoDeLoja>("SELECT * FROM avaliacoes_loja WHERE usuario_uuid = $1 AND loja_uuid = $2")
            .bind(usuario_uuid)
            .bind(loja_uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))
    }

    async fn atualizar(&self, avaliacao: AvaliacaoDeLoja) -> DomainResult<()> {
        let uuid = avaliacao.uuid;
        let result = sqlx::query(
            "UPDATE avaliacoes_loja SET nota = $1, comentario = $2 WHERE uuid = $3"
        )
        .bind(avaliacao.nota)
        .bind(&avaliacao.comentario)
        .bind(uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            Err(DomainError::NotFound {
                entity: "Avaliação",
                id: uuid.to_string(),
            })
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> DomainResult<()> {
        let result = sqlx::query("DELETE FROM avaliacoes_loja WHERE uuid = $1")
            .bind(uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            Err(DomainError::NotFound {
                entity: "Avaliação",
                id: uuid.to_string(),
            })
        } else {
            Ok(())
        }
    }
}
