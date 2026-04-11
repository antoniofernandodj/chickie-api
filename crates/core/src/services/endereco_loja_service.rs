use std::sync::Arc;
use uuid::Uuid;

use crate::models::EnderecoLoja;
use crate::ports::EnderecoLojaRepositoryPort;

#[derive(Clone)]
pub struct EnderecoLojaService {
    repo: Arc<dyn EnderecoLojaRepositoryPort>,
}

impl EnderecoLojaService {
    pub fn new(repo: Arc<dyn EnderecoLojaRepositoryPort>) -> Self {
        Self { repo }
    }

    pub async fn listar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<EnderecoLoja>, String> {
        self.repo.listar_por_loja(loja_uuid).await.map_err(|e| e.to_string())
    }

    pub async fn criar(&self, endereco: &EnderecoLoja) -> Result<Uuid, String> {
        self.repo.criar(endereco).await.map_err(|e| e.to_string())
    }

    pub async fn atualizar(&self, endereco: EnderecoLoja) -> Result<(), String> {
        self.repo.atualizar(endereco).await.map_err(|e| e.to_string())
    }

    pub async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        self.repo.deletar(uuid).await.map_err(|e| e.to_string())
    }
}
