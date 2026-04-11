use std::sync::Arc;
use uuid::Uuid;

use crate::models::EnderecoLoja;
use crate::repositories::{EnderecoLojaRepository, Repository};

#[derive(Clone)]
pub struct EnderecoLojaService {
    repo: Arc<EnderecoLojaRepository>,
}

impl EnderecoLojaService {
    pub fn new(repo: Arc<EnderecoLojaRepository>) -> Self {
        Self { repo }
    }

    pub async fn listar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<EnderecoLoja>, String> {
        self.repo.buscar_por_loja(loja_uuid).await
    }

    pub async fn criar(&self, endereco: &EnderecoLoja) -> Result<Uuid, String> {
        self.repo.criar(endereco).await
    }

    pub async fn atualizar(&self, endereco: EnderecoLoja) -> Result<(), String> {
        self.repo.atualizar(endereco).await
    }

    pub async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        self.repo.deletar(uuid).await
    }
}
