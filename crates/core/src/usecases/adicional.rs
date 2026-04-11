use std::sync::Arc;

use uuid::Uuid;

use crate::models::{Adicional, Usuario};
use crate::repositories::AdicionalRepository;

pub struct AdicionalUsecase {
    repo: Arc<AdicionalRepository>,
    usuario: Usuario,
    loja_uuid: Uuid,
}

impl AdicionalUsecase {
    pub fn new(
        repo: Arc<AdicionalRepository>,
        usuario: Usuario,
        loja_uuid: Uuid,
    ) -> Self {
        Self { repo, usuario, loja_uuid }
    }

    pub async fn listar_todos(&self) -> Result<Vec<Adicional>, String> {
        self.repo.buscar_por_loja(self.loja_uuid).await
    }

    pub async fn listar_disponiveis(&self) -> Result<Vec<Adicional>, String> {
        self.repo.buscar_disponiveis(self.loja_uuid).await
    }

    pub async fn marcar_indisponivel(&self, adicional_uuid: Uuid) -> Result<(), String> {
        // Verifica que o adicional pertence à loja
        let adicionais = self.repo.buscar_por_loja(self.loja_uuid).await?;
        let existe = adicionais.iter().find(|a| a.uuid == adicional_uuid);

        if existe.is_none() {
            return Err("Adicional não encontrado nesta loja".to_string());
        }

        self.repo.marcar_indisponivel(adicional_uuid).await
    }
}
