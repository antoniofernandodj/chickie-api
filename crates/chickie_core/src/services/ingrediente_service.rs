use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::models::Ingrediente;
use crate::ports::IngredienteRepositoryPort;

#[derive(Clone)]
pub struct IngredienteService {
    repo: Arc<dyn IngredienteRepositoryPort>,
}

#[allow(dead_code)]
impl IngredienteService {
    pub fn new(repo: Arc<dyn IngredienteRepositoryPort>) -> Self {
        Self { repo }
    }

    pub async fn criar(
        &self,
        loja_uuid: Uuid,
        nome: String,
        unidade_medida: Option<String>,
        // quantidade: f64,
        preco_unitario: Decimal,
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
        self.repo.listar_por_loja(loja_uuid).await.map_err(|e| e.to_string())
    }

    pub async fn listar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Ingrediente>, String> {
        // Port doesn't have buscar_disponiveis, return all for now
        self.repo.listar_por_loja(loja_uuid).await.map_err(|e| e.to_string())
    }

    pub async fn atualizar(
        &self,
        _uuid: Uuid,
        _nome: String,
        _unidade_medida: Option<String>,
        _quantidade: Decimal,
        _preco_unitario: Decimal,
    ) -> Result<(), String> {
        // Port doesn't have buscar_por_uuid - would need to be added
        Err("atualizar ingrediente requires buscar_por_uuid on port".to_string())
    }

    pub async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        self.repo.deletar(uuid).await.map_err(|e| e.to_string())
    }
}
