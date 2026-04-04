use std::sync::Arc;
use uuid::Uuid;

use crate::models::Entregador;
use crate::repositories::{EntregadorRepository, Repository as _};

#[derive(Clone)]
pub struct EntregadorService {
    repo: Arc<EntregadorRepository>,
}

impl EntregadorService {
    pub fn new(repo: Arc<EntregadorRepository>) -> Self {
        Self { repo }
    }

    pub async fn listar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Entregador>, String> {
        self.repo.buscar_por_loja(loja_uuid).await
    }

    pub async fn listar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Entregador>, String> {
        self.repo.buscar_disponiveis(loja_uuid).await
    }

    pub async fn atualizar(
        &self,
        uuid: Uuid,
        veiculo: Option<String>,
        placa: Option<String>,
    ) -> Result<(), String> {
        let mut entregador = self.repo.buscar_por_uuid(uuid).await?
            .ok_or("Entregador não encontrado")?;
        entregador.veiculo = veiculo;
        entregador.placa = placa;
        self.repo.atualizar(entregador).await
    }

    pub async fn definir_disponivel(&self, uuid: Uuid, disponivel: bool) -> Result<(), String> {
        let mut entregador = self.repo.buscar_por_uuid(uuid).await?
            .ok_or("Entregador não encontrado")?;
        entregador.disponivel = disponivel;
        self.repo.atualizar(entregador).await
    }

    pub async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        self.repo.deletar(uuid).await
    }
}
