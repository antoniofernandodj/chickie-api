use std::sync::Arc;
use uuid::Uuid;
use chrono::{FixedOffset, Timelike, Utc, Datelike};
use serde::Serialize;

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

    pub async fn verificar_aberta_agora(&self, loja_uuid: Uuid) -> Result<StatusLoja, String> {
        // Brasília = UTC-3 (sem horário de verão desde 2019)
        let brasilia = FixedOffset::west_opt(3 * 3600).unwrap();
        let agora = Utc::now().with_timezone(&brasilia);
        let hora_atual = agora.time().with_second(0).unwrap().with_nanosecond(0).unwrap();
        // 0=Domingo, 1=Segunda, ..., 6=Sábado
        let dia_semana = agora.weekday().num_days_from_sunday() as i32;

        let horario = self.repo
            .buscar_por_loja_e_dia(loja_uuid, dia_semana)
            .await
            .map_err(|e| e.to_string())?;

        let Some(h) = horario else {
            return Ok(StatusLoja {
                aberta: false,
                hora_atual: format!("{:02}:{:02}", agora.hour(), agora.minute()),
                dia_semana,
                abertura: None,
                fechamento: None,
            });
        };

        if !h.ativo {
            return Ok(StatusLoja {
                aberta: false,
                hora_atual: format!("{:02}:{:02}", agora.hour(), agora.minute()),
                dia_semana,
                abertura: Some(h.abertura.format("%H:%M").to_string()),
                fechamento: Some(h.fechamento.format("%H:%M").to_string()),
            });
        }

        let aberta = hora_atual >= h.abertura && hora_atual < h.fechamento;

        Ok(StatusLoja {
            aberta,
            hora_atual: format!("{:02}:{:02}", agora.hour(), agora.minute()),
            dia_semana,
            abertura: Some(h.abertura.format("%H:%M").to_string()),
            fechamento: Some(h.fechamento.format("%H:%M").to_string()),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct StatusLoja {
    pub aberta: bool,
    pub hora_atual: String,
    pub dia_semana: i32,
    pub abertura: Option<String>,
    pub fechamento: Option<String>,
}
