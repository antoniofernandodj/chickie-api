use super::CronJob;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{info, error};

use chickie_core::database::criar_pool;
use chickie_core::repositories::{UsuarioRepository, LojaRepository};
use chickie_core::adapters::{UsuarioRepositoryAdapter, LojaRepositoryAdapter};
use chickie_core::services::SoftDeleteService;

pub struct SoftDeleteCleanupJob;

#[async_trait]
impl CronJob for SoftDeleteCleanupJob {
    fn name(&self) -> &'static str {
        "soft_delete_cleanup_job"
    }

    async fn execute(&self) -> Result<()> {
        info!("🧹 Iniciando limpeza de soft deletes pendentes...");

        // Pool via core crate (mesma função usada pela API)
        let pool = Arc::new(
            criar_pool()
                .await
                .map_err(|e| anyhow::anyhow!("Falha ao criar pool: {}", e))?
        );

        // Repositórios concretos
        let usuario_repo =
            Arc::new(UsuarioRepository::new(pool.clone()));
        let loja_repo =
            Arc::new(LojaRepository::new(pool.clone()));

        // Adapters (concretos → ports)
        let usuario_port =
            Arc::new(UsuarioRepositoryAdapter::from_repo(usuario_repo));
        let loja_port =
            Arc::new(LojaRepositoryAdapter::from_repo(loja_repo));

        // Service (camada de negócio)
        let service = SoftDeleteService::new(usuario_port, loja_port);

        // Executar deleção de pendentes > 30 dias
        match service.deletar_pendentes_antigos().await {
            Ok((usuarios_deletados, lojas_deletadas)) => {
                if usuarios_deletados > 0 {
                    info!(
                        "✅ {} usuário(s) deletado(s) permanentemente (> 30 dias)",
                        usuarios_deletados
                    );
                } else {
                    info!("⏭️  Nenhum usuário pendente para deleção permanente");
                }

                if lojas_deletadas > 0 {
                    info!(
                        "✅ {} loja(s) deletada(s) permanentemente (> 30 dias)",
                        lojas_deletadas
                    );
                } else {
                    info!("⏭️  Nenhuma loja pendente para deleção permanente");
                }
            }
            Err(e) => {
                error!("❌ Falha na limpeza de soft deletes: {}", e);
            }
        }

        info!("✨ Limpeza de soft deletes concluída");

        Ok(())
    }
}