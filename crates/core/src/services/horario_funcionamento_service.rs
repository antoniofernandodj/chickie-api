use std::sync::Arc;
use uuid::Uuid;

use crate::models::HorarioFuncionamento;
use crate::ports::HorarioFuncionamentoRepositoryPort;

#[derive(Clone)]
pub struct HorarioFuncionamentoService {
    repo: Arc<dyn HorarioFuncionamentoRepositoryPort>,
}

impl HorarioFuncionamentoService {
    pub fn new(repo: Arc<dyn HorarioFuncionamentoRepositoryPort>) -> Self {
        Self { repo }
    }

    pub async fn listar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<HorarioFuncionamento>, String> {
        self.repo.listar_por_loja(loja_uuid).await.map_err(|e| e.to_string())
    }

    pub async fn criar_ou_atualizar(&self, horario: &HorarioFuncionamento) -> Result<(), String> {
        self.repo.adicionar_sem_sobrescrever(horario).await.map_err(|e| e.to_string())
    }

    pub async fn definir_ativo(&self, loja_uuid: Uuid, dia_semana: i32, ativo: bool) -> Result<(), String> {
        self.repo.definir_ativo(loja_uuid, dia_semana, ativo).await.map_err(|e| e.to_string())
    }

    pub async fn deletar_por_dia(&self, loja_uuid: Uuid, dia_semana: i32) -> Result<(), String> {
        self.repo.deletar_por_dia(loja_uuid, dia_semana).await.map_err(|e| e.to_string())
    }
}
