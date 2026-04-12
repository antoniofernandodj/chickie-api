use super::CronJob;
use anyhow::Result;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use sqlx::postgres::PgPoolOptions;
use tracing::{info, error};

pub struct SoftDeleteCleanupJob;

#[async_trait]
impl CronJob for SoftDeleteCleanupJob {
    fn name(&self) -> &'static str {
        "soft_delete_cleanup_job"
    }

    async fn execute(&self) -> Result<()> {
        info!("🧹 Iniciando limpeza de soft deletes pendentes...");

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|e| anyhow::anyhow!("DATABASE_URL não encontrado: {}", e))?;

        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(&database_url)
            .await
            .map_err(|e| anyhow::anyhow!("Falha ao conectar ao banco: {}", e))?;

        // ===================================================================
        // USUARIOS: marcar como deletados os que estão pendentes há > 30 dias
        // ===================================================================
        let thirty_days_ago = Utc::now() - Duration::days(30);

        let result_usuarios = sqlx::query(
            "UPDATE usuarios 
             SET deletado = true, atualizado_em = NOW() 
             WHERE marcado_para_remocao IS NOT NULL 
               AND marcado_para_remocao <= $1 
               AND deletado = false"
        )
        .bind(thirty_days_ago)
        .execute(&pool)
        .await;

        match result_usuarios {
            Ok(res) => {
                let count = res.rows_affected();
                if count > 0 {
                    info!("✅ {} usuário(s) deletado(s) permanentemente (marcados há mais de 30 dias)", count);
                } else {
                    info!("⏭️  Nenhum usuário pendente para deleção permanente");
                }
            }
            Err(e) => {
                error!("❌ Falha ao deletar usuários pendentes: {}", e);
            }
        }

        // ===================================================================
        // LOJAS: marcar como deletadas as que estão pendentes há > 30 dias
        // ===================================================================
        let result_lojas = sqlx::query(
            "UPDATE lojas 
             SET deletado = true, atualizado_em = NOW() 
             WHERE marcado_para_remocao IS NOT NULL 
               AND marcado_para_remocao <= $1 
               AND deletado = false"
        )
        .bind(thirty_days_ago)
        .execute(&pool)
        .await;

        match result_lojas {
            Ok(res) => {
                let count = res.rows_affected();
                if count > 0 {
                    info!("✅ {} loja(s) deletada(s) permanentemente (marcadas há mais de 30 dias)", count);
                } else {
                    info!("⏭️  Nenhuma loja pendente para deleção permanente");
                }
            }
            Err(e) => {
                error!("❌ Falha ao deletar lojas pendentes: {}", e);
            }
        }

        // ===================================================================
        // Log de entidades ainda pendentes (para monitoramento)
        // ===================================================================
        let pending_users = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM usuarios WHERE marcado_para_remocao IS NOT NULL AND deletado = false"
        )
        .fetch_one(&pool)
        .await
        .unwrap_or(-1);

        let pending_lojas = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM lojas WHERE marcado_para_remocao IS NOT NULL AND deletado = false"
        )
        .fetch_one(&pool)
        .await
        .unwrap_or(-1);

        if pending_users > 0 || pending_lojas > 0 {
            info!(
                "📊 Entidades aguardando deleção (< 30 dias): {} usuário(s), {} loja(s)",
                pending_users, pending_lojas
            );
        }

        info!("✨ Limpeza de soft deletes concluída");

        Ok(())
    }
}