use super::CronJob;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{info, error};

use chickie_core::database::criar_pool;
use chickie_core::ports::PreCadastroPort;
use chickie_core::repositories::PreCadastroRepository;

pub struct CleanupJob;

#[async_trait]
impl CronJob for CleanupJob {
    fn name(&self) -> &'static str {
        "cleanup_job"
    }

    async fn execute(&self) -> Result<()> {
        info!("🧹 Iniciando limpeza de pré-cadastros expirados...");

        let pool = Arc::new(
            criar_pool()
                .await
                .map_err(|e| anyhow::anyhow!("Falha ao criar pool: {}", e))?
        );

        let repo = PreCadastroRepository::new(pool);

        match repo.limpar_expirados().await {
            Ok(0) => info!("⏭️  Nenhum pré-cadastro expirado para remover"),
            Ok(n) => info!("✅ {} pré-cadastro(s) expirado(s) removido(s)", n),
            Err(e) => error!("❌ Falha ao limpar pré-cadastros expirados: {}", e),
        }

        info!("✨ Limpeza concluída");
        Ok(())
    }
}
