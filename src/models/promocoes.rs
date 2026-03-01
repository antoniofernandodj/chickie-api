use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

use crate::{models::Model, utils::agora};

// --- StatusCupom ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusCupom {
    Ativo,
    Inativo,
    Expirado,
    Esgotado,
}

impl StatusCupom {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ativo    => "ativo",
            Self::Inativo  => "inativo",
            Self::Expirado => "expirado",
            Self::Esgotado => "esgotado",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "ativo"    => Ok(Self::Ativo),
            "inativo"  => Ok(Self::Inativo),
            "expirado" => Ok(Self::Expirado),
            "esgotado" => Ok(Self::Esgotado),
            other => Err(format!("Status inválido: {}", other)),
        }
    }
}

impl std::fmt::Display for StatusCupom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for StatusCupom {
    fn decode(value: sqlx::sqlite::SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
        Self::from_str(&s).map_err(|e| e.into())
    }
}

impl sqlx::Type<sqlx::Sqlite> for StatusCupom {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}

impl sqlx::Encode<'_, sqlx::Sqlite> for StatusCupom {
    fn encode_by_ref(&self, args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'_>>) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        args.push(sqlx::sqlite::SqliteArgumentValue::Text(
            self.as_str().to_string().into()
        ));
        Ok(sqlx::encode::IsNull::No)
    }
}

// --- Cupom ---
// tipo_desconto: "percentual", "valor_fixo", "frete_gratis"
// valor_desconto: o valor/percentual (NULL para frete_gratis)

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Cupom {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub codigo: String,
    pub descricao: String,
    pub tipo_desconto: String,
    pub valor_desconto: Option<f64>,
    pub valor_minimo: Option<f64>,
    pub data_validade: String,
    pub limite_uso: Option<i32>,
    pub uso_atual: i32,
    pub status: StatusCupom,
    pub criado_em: String,
}

impl Cupom {
    pub fn new(
        loja_uuid: Uuid,
        codigo: String,
        descricao: String,
        tipo_desconto: String,
        valor_desconto: Option<f64>,
        valor_minimo: Option<f64>,
        data_validade: String,
        limite_uso: Option<i32>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            codigo: codigo.to_uppercase(),
            descricao,
            tipo_desconto,
            valor_desconto,
            valor_minimo,
            data_validade,
            limite_uso,
            uso_atual: 0,
            status: StatusCupom::Ativo,
            criado_em: agora()
        }
    }

    pub fn calcular_desconto(&self, valor_pedido: f64, valor_frete: f64) -> f64 {
        match self.tipo_desconto.as_str() {
            "percentual"   => valor_desconto_com_limite(
                valor_pedido * (self.valor_desconto.unwrap_or(0.0) / 100.0),
                None,
            ),
            "valor_fixo"   => self.valor_desconto.unwrap_or(0.0),
            "frete_gratis" => valor_frete,
            _              => 0.0,
        }
    }

    pub fn registrar_uso(&mut self) {
        self.uso_atual += 1;
        if let Some(limite) = self.limite_uso {
            if self.uso_atual >= limite {
                self.status = StatusCupom::Esgotado;
            }
        }
    }

    pub fn ativar(&mut self) {
        if self.status == StatusCupom::Inativo {
            self.status = StatusCupom::Ativo;
        }
    }

    pub fn desativar(&mut self) {
        self.status = StatusCupom::Inativo;
    }
}

fn valor_desconto_com_limite(desconto: f64, maximo: Option<f64>) -> f64 {
    if let Some(max) = maximo {
        desconto.min(max)
    } else {
        desconto
    }
}

// --- UsoCupom ---

#[derive(Debug, Clone, FromRow)]
pub struct UsoCupom {
    pub uuid: Uuid,
    pub cupom_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub pedido_uuid: Uuid,
    pub valor_desconto: f64,
    pub usado_em: String,
}

impl UsoCupom {
    pub fn new(
        cupom_uuid: Uuid,
        usuario_uuid: Uuid,
        pedido_uuid: Uuid,
        valor_desconto: f64,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            cupom_uuid,
            usuario_uuid,
            pedido_uuid,
            valor_desconto,
            usado_em: agora(),
        }
    }
}


#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Promocao {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub descricao: String,
    pub tipo_desconto: String,       // "percentual", "valor_fixo", "frete_gratis"
    pub valor_desconto: Option<f64>,
    pub valor_minimo: Option<f64>,
    pub data_inicio: String,
    pub data_fim: String,
    pub dias_semana_validos: Option<String>, // "0,1,2,3,4,5,6" serializado como CSV
    pub status: StatusCupom,
    pub prioridade: i32,
    pub criado_em: String,
}

impl Promocao {
    pub fn new(
        loja_uuid: Uuid,
        nome: String,
        descricao: String,
        tipo_desconto: String,
        valor_desconto: Option<f64>,
        valor_minimo: Option<f64>,
        data_inicio: String,
        data_fim: String,
        dias_semana_validos: Option<Vec<u8>>,
        prioridade: i32,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            nome,
            descricao,
            tipo_desconto,
            valor_desconto,
            valor_minimo,
            data_inicio,
            data_fim,
            dias_semana_validos: dias_semana_validos
                .map(|d| d.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",")),
            status: StatusCupom::Ativo,
            prioridade,
            criado_em: agora()
        }
    }

    pub fn eh_aplicavel(&self, valor_pedido: f64, data_hora: String, dia_semana: u8) -> bool {
        if self.status != StatusCupom::Ativo {
            return false;
        }
        if data_hora < self.data_inicio || data_hora > self.data_fim {
            return false;
        }
        if let Some(minimo) = self.valor_minimo {
            if valor_pedido < minimo {
                return false;
            }
        }
        if let Some(ref dias_str) = self.dias_semana_validos {
            let dias: Vec<u8> = dias_str
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            if !dias.contains(&dia_semana) {
                return false;
            }
        }
        true
    }

    pub fn calcular_desconto(&self, valor_pedido: f64, valor_frete: f64) -> f64 {
        match self.tipo_desconto.as_str() {
            "percentual"   => valor_pedido * (self.valor_desconto.unwrap_or(0.0) / 100.0),
            "valor_fixo"   => self.valor_desconto.unwrap_or(0.0),
            "frete_gratis" => valor_frete,
            _              => 0.0,
        }
    }

    pub fn ativar(&mut self) {
        if self.status == StatusCupom::Inativo {
            self.status = StatusCupom::Ativo;
        }
    }

    pub fn desativar(&mut self) {
        self.status = StatusCupom::Inativo;
    }
}


impl Model for Promocao {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}

impl Model for Cupom {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}

impl Model for UsoCupom {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
