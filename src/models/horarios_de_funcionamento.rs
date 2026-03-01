use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{models::Model, utils::agora};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct HorarioFuncionamento {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub dia_semana: i32,      // 0=Domingo, 1=Segunda, ..., 6=Sábado
    pub abertura: String,     // "HH:MM"
    pub fechamento: String,   // "HH:MM"
    pub ativo: bool,
    pub criado_em: String,
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
        
        fn valida_hora(h: &str) -> bool {
            let partes: Vec<&str> = h.split(':').collect();
            if partes.len() != 2 { return false; }
            let (hh, mm) = (
                partes[0].parse::<u8>(), partes[1].parse::<u8>()
            );
            matches!((hh, mm), (Ok(h), Ok(m)) if h < 24 && m < 60)
        }

        if !valida_hora(&abertura) {
            return Err(format!("Horário de abertura inválido: {}", abertura));
        }
        if !valida_hora(&fechamento) {
            return Err(format!("Horário de fechamento inválido: {}", fechamento));
        }

        Ok(Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            dia_semana,
            abertura,
            fechamento,
            ativo: true,
            criado_em: agora(),
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
