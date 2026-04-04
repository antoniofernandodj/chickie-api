use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use chrono::{Utc, NaiveTime};

use crate::models::Model;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct HorarioFuncionamento {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub dia_semana: i32,      // 0=Domingo, 1=Segunda, ..., 6=Sábado
    pub abertura: NaiveTime,  // "HH:MM"
    pub fechamento: NaiveTime, // "HH:MM"
    pub ativo: bool,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl HorarioFuncionamento {
    pub fn new(
        loja_uuid: Uuid,
        dia_semana: i32,
        abertura: String,
        fechamento: String,
    ) -> Result<Self, String> {
        if !(0..=6).contains(&dia_semana) {
            return Err("dia_semana deve ser entre 0 (Domingo) e 6 (Sábado)".into());
        }

        let abertura = NaiveTime::parse_from_str(&abertura, "%H:%M")
            .map_err(|e| format!("Horário de abertura inválido '{}': {}", abertura, e))?;

        let fechamento = NaiveTime::parse_from_str(&fechamento, "%H:%M")
            .map_err(|e| format!("Horário de fechamento inválido '{}': {}", fechamento, e))?;

        Ok(Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            dia_semana,
            abertura,
            fechamento,
            ativo: true,
            criado_em: Utc::now(),
        })
    }

    pub fn nome_dia(&self) -> &'static str {
        match self.dia_semana {
            0 => "Domingo",
            1 => "Segunda-feira",
            2 => "Terça-feira",
            3 => "Quarta-feira",
            4 => "Quinta-feira",
            5 => "Sexta-feira",
            6 => "Sábado",
            _ => "Desconhecido",
        }
    }
}


impl Model for HorarioFuncionamento {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
