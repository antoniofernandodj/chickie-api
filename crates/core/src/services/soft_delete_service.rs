use std::sync::Arc;
use chrono::{Duration, Utc};
use crate::ports::{UsuarioRepositoryPort, LojaRepositoryPort};

pub struct SoftDeleteService {
    usuario_port: Arc<dyn UsuarioRepositoryPort>,
    loja_port: Arc<dyn LojaRepositoryPort>,
}

impl SoftDeleteService {
    pub fn new(
        usuario_port: Arc<dyn UsuarioRepositoryPort>,
        loja_port: Arc<dyn LojaRepositoryPort>,
    ) -> Self {
        Self { usuario_port, loja_port }
    }

    /// Deleta permanentemente todas as entidades (usuarios + lojas) marcadas para remoção há mais de 30 dias.
    /// Retorna (usuarios_deletados, lojas_deletadas).
    pub async fn deletar_pendentes_antigos(&self) -> Result<(u64, u64), String> {
        let thirty_days_ago = Utc::now() - Duration::days(30);

        let usuarios_deletados = self.usuario_port
            .deletar_pendentes_antigos(thirty_days_ago)
            .await
            .map_err(|e| format!("Erro ao deletar usuários: {}", e))?;

        let lojas_deletadas = self.loja_port
            .deletar_pendentes_antigas(thirty_days_ago)
            .await
            .map_err(|e| format!("Erro ao deletar lojas: {}", e))?;

        Ok((usuarios_deletados, lojas_deletadas))
    }
}