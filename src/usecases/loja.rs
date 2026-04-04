use std::sync::Arc;
use crate::models::Loja;
use crate::services::LojaService;

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
}
