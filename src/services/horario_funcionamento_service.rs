use std::sync::Arc;
use uuid::Uuid;
use chrono::NaiveTime;

use crate::entities::horarios_funcionamento::Model as HorarioFuncionamento;
use crate::repositories::{HorarioFuncionamentoRepository};

#[derive(Clone)]
pub struct HorarioFuncionamentoService {
    repo: Arc<HorarioFuncionamentoRepository>,
}

impl HorarioFuncionamentoService {
    pub fn new(repo: Arc<HorarioFuncionamentoRepository>) -> Self {
        Self { repo }
    }

    pub async fn listar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<HorarioFuncionamento>, String> {
        self.repo.buscar_por_loja(loja_uuid).await
    }

    pub async fn criar_ou_atualizar(&self, horario: &HorarioFuncionamento) -> Result<(), String> {
        self.repo.adicionar_ou_atualizar(horario).await
    }

    pub async fn criar_horario(
        &self,
        loja_uuid: Uuid,
        dia_semana: i32,
        abertura: NaiveTime,
        fechamento: NaiveTime,
    ) -> Result<HorarioFuncionamento, String> {
        let horario = HorarioFuncionamento {
            uuid: Uuid::new_v4(),
            loja_uuid,
            dia_semana,
            abertura,
            fechamento,
            ativo: true,
            criado_em: chrono::Utc::now(),
        };
        self.repo.adicionar_sem_sobrescrever(&horario).await?;
        Ok(horario)
    }

    pub async fn definir_ativo(&self, loja_uuid: Uuid, dia_semana: i32, ativo: bool) -> Result<(), String> {
        self.repo.definir_ativo(loja_uuid, dia_semana, ativo).await
    }

    pub async fn deletar_por_dia(&self, loja_uuid: Uuid, dia_semana: i32) -> Result<(), String> {
        self.repo.deletar_por_dia(loja_uuid, dia_semana).await
    }
}
