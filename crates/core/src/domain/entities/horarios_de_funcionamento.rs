use uuid::Uuid;
use chrono::{DateTime, NaiveTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct HorarioFuncionamento {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub dia_semana: i32,
    pub abertura: NaiveTime,
    pub fechamento: NaiveTime,
    pub ativo: bool,
    pub criado_em: DateTime<Utc>,
}

impl HorarioFuncionamento {
    pub fn new(loja_uuid: Uuid, dia_semana: i32, abertura: String, fechamento: String) -> Result<Self, String> {
        if !(0..=6).contains(&dia_semana) { return Err("dia_semana deve ser entre 0 (Domingo) e 6 (Sabado)".into()); }
        let abertura = NaiveTime::parse_from_str(&abertura, "%H:%M").map_err(|e| format!("Horario de abertura invalido '{}': {}", abertura, e))?;
        let fechamento = NaiveTime::parse_from_str(&fechamento, "%H:%M").map_err(|e| format!("Horario de fechamento invalido '{}': {}", fechamento, e))?;
        Ok(Self { uuid: Uuid::new_v4(), loja_uuid, dia_semana, abertura, fechamento, ativo: true, criado_em: Utc::now() })
    }

    pub fn nome_dia(&self) -> &'static str {
        match self.dia_semana {
            0 => "Domingo", 1 => "Segunda-feira", 2 => "Terca-feira", 3 => "Quarta-feira",
            4 => "Quinta-feira", 5 => "Sexta-feira", 6 => "Sabado", _ => "Desconhecido",
        }
    }
}
