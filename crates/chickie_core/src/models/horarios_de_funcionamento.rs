use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use chrono::{Utc, NaiveTime};
use utoipa::ToSchema;
use crate::{models::Model, ports::to_proto::ToProto};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct HorarioFuncionamento {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub dia_semana: i32,
    pub abertura: NaiveTime,
    pub fechamento: NaiveTime,
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

impl ToProto<crate::proto::HorarioFuncionamento> for HorarioFuncionamento {
    fn to_proto(&self) -> crate::proto::HorarioFuncionamento {
        crate::proto::HorarioFuncionamento {
            uuid: self.uuid.to_string(),
            loja_uuid: self.loja_uuid.to_string(),
            dia_semana: self.dia_semana,
            abertura: self.abertura.format("%H:%M").to_string(),
            fechamento: self.fechamento.format("%H:%M").to_string(),
            ativo: self.ativo,
            criado_em: self.criado_em.to_rfc3339(),
        }
    }
}

impl Model for HorarioFuncionamento {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
