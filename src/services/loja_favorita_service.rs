use std::sync::Arc;
use uuid::Uuid;

use crate::entities::loja_favorita::Model as LojaFavorita;
use crate::repositories::{LojaFavoritaRepository, Repository as _};

#[derive(Clone)]
pub struct LojaFavoritaService {
    repo: Arc<LojaFavoritaRepository>,
}

impl LojaFavoritaService {
    pub fn new(repo: Arc<LojaFavoritaRepository>) -> Self {
        Self { repo }
    }

    /// Adiciona uma loja como favorita para um usuário
    pub async fn adicionar_favorita(
        &self,
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
    ) -> Result<LojaFavorita, String> {

        // Verifica se já não está favoritada
        if self.repo.buscar_por_usuario_e_loja(usuario_uuid, loja_uuid).await?.is_some() {
            return Err("Loja já está na lista de favoritas".to_string());
        }

        let favorita = LojaFavorita {
            uuid: Uuid::new_v4(),
            usuario_uuid,
            loja_uuid,
            criado_em: chrono::Utc::now(),
        };
        self.repo.criar(&favorita).await?;
        Ok(favorita)
    }

    /// Remove uma loja da lista de favoritas de um usuário
    pub async fn remover_favorita(
        &self,
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
    ) -> Result<(), String> {

        let existente = self.repo.buscar_por_usuario_e_loja(usuario_uuid, loja_uuid).await?
            .ok_or("Loja não está na lista de favoritas")?;

        self.repo.deletar(existente.uuid).await
    }

    /// Lista todas as lojas favoritas de um usuário
    pub async fn listar_favoritas(&self, usuario_uuid: Uuid) -> Result<Vec<LojaFavorita>, String> {
        self.repo.buscar_por_usuario(usuario_uuid).await
    }

    /// Verifica se uma loja é favorita para um usuário
    pub async fn eh_favorita(&self, usuario_uuid: Uuid, loja_uuid: Uuid) -> Result<bool, String> {
        let result = self.repo.buscar_por_usuario_e_loja(usuario_uuid, loja_uuid).await?;
        Ok(result.is_some())
    }
}
