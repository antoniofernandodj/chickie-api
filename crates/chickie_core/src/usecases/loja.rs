use std::sync::Arc;

use uuid::Uuid;

use crate::{models::Loja, services::LojaService};

pub struct LojaUsecase {
    loja_service: Arc<LojaService>,
}

impl LojaUsecase {
    pub fn new(loja_service: Arc<LojaService>) -> Self {
        Self { loja_service }
    }

    pub async fn pesquisar_lojas(&self, termo: &str) -> Result<Vec<Loja>, String> {
        self.loja_service.pesquisar(termo).await
    }

    pub async fn buscar_loja(&self, uuid: Uuid) -> Result<Loja, String> {
        self.loja_service
            .buscar_por_uuid(uuid)
            .await?
            .ok_or_else(|| "Loja não encontrada".to_string())
    }

    pub async fn buscar_loja_por_slug(&self, slug: &str) -> Result<Loja, String> {
        self.loja_service
            .buscar_por_slug(slug)
            .await?
            .ok_or_else(|| "Loja não encontrada".to_string())
    }

    pub async fn verificar_slug_disponivel(&self, slug: &str) -> Result<bool, String> {
        self.loja_service.verificar_slug_disponivel(slug).await
    }
}
