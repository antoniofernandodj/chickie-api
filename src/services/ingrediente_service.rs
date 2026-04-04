use std::sync::Arc;
use uuid::Uuid;

use crate::models::Ingrediente;
use crate::repositories::{IngredienteRepository, Repository as _};

#[derive(Clone)]
pub struct IngredienteService {
    repo: Arc<IngredienteRepository>,
}

impl IngredienteService {
    pub fn new(repo: Arc<IngredienteRepository>) -> Self {
        Self { repo }
    }

    pub async fn criar(
        &self,
        loja_uuid: Uuid,
        nome: String,
        unidade_medida: Option<String>,
        // quantidade: f64,
        preco_unitario: f64,
    ) -> Result<Ingrediente, String> {
        let ingrediente = Ingrediente::new(
            nome,
            loja_uuid,
            unidade_medida,
            preco_unitario,
        );
        self.repo.criar(&ingrediente).await?;
        Ok(ingrediente)
    }

    pub async fn listar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Ingrediente>, String> {
        self.repo.buscar_por_loja(loja_uuid).await
    }

    pub async fn listar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Ingrediente>, String> {
        self.repo.buscar_disponiveis(loja_uuid).await
    }

    pub async fn atualizar(
        &self,
        uuid: Uuid,
        nome: String,
        unidade_medida: Option<String>,
        quantidade: f64,
        preco_unitario: f64,
    ) -> Result<(), String> {
        let mut ingrediente = self.repo.buscar_por_uuid(uuid).await?
            .ok_or("Ingrediente não encontrado")?;
        ingrediente.nome = nome;
        ingrediente.unidade_medida = unidade_medida;
        ingrediente.quantidade = quantidade;
        ingrediente.preco_unitario = preco_unitario;
        self.repo.atualizar(ingrediente).await
    }

    pub async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        self.repo.deletar(uuid).await
    }
}
