use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::Utc;
use rust_decimal::Decimal;
use utoipa::ToSchema;

use crate::{models::Model, ports::to_proto::ToProto};

// --- TipoEscopoPromocao ---

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum TipoEscopoPromocao {
    Loja,
    Produto,
    Categoria,
}

impl TipoEscopoPromocao {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Loja => "loja",
            Self::Produto => "produto",
            Self::Categoria => "categoria",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "loja" => Ok(Self::Loja),
            "produto" => Ok(Self::Produto),
            "categoria" => Ok(Self::Categoria),
            other => Err(format!("Escopo inválido: '{}'", other)),
        }
    }
}

impl std::fmt::Display for TipoEscopoPromocao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for TipoEscopoPromocao {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Self::from_str(s).map_err(|e| e.into())
    }
}

impl sqlx::Type<sqlx::Postgres> for TipoEscopoPromocao {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("TEXT")
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for TipoEscopoPromocao {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        <&str as sqlx::Encode<sqlx::Postgres>>::encode(self.as_str(), buf)
    }

    fn produces(&self) -> Option<sqlx::postgres::PgTypeInfo> {
        Some(<Self as sqlx::Type<sqlx::Postgres>>::type_info())
    }
}

// --- StatusCupom ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
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

// ✅ IMPLEMENTAÇÕES PARA POSTGRESQL

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for StatusCupom {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>
    ) -> Result<Self, sqlx::error::BoxDynError> {
        // PostgreSQL retorna &str para tipo TEXT
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Self::from_str(s).map_err(|e| e.into())
    }
}

impl sqlx::Type<sqlx::Postgres> for StatusCupom {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        // Mapeia para TEXT no PostgreSQL
        sqlx::postgres::PgTypeInfo::with_name("TEXT")
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for StatusCupom {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        // Encode como TEXT usando o encoder nativo de &str
        <&str as sqlx::Encode<sqlx::Postgres>>::encode(self.as_str(), buf)
    }

    fn produces(&self) -> Option<sqlx::postgres::PgTypeInfo> {
        Some(<Self as sqlx::Type<sqlx::Postgres>>::type_info())
    }
}

// --- Cupom ---
// tipo_desconto: "percentual", "valor_fixo", "frete_gratis"
// valor_desconto: o valor/percentual (NULL para frete_gratis)

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Cupom {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub codigo: String,
    pub descricao: String,
    pub tipo_desconto: String,
    pub valor_desconto: Option<Decimal>,
    pub valor_minimo: Option<Decimal>,
    pub data_validade: chrono::DateTime<chrono::Utc>,
    pub limite_uso: Option<i32>,
    pub uso_atual: i32,
    pub status: StatusCupom,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl Cupom {
    pub fn new(
        loja_uuid: Uuid,
        codigo: String,
        descricao: String,
        tipo_desconto: String,
        valor_desconto: Option<Decimal>,
        valor_minimo: Option<Decimal>,
        data_validade: chrono::DateTime<chrono::Utc>,
        limite_uso: Option<i32>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            codigo,
            descricao,
            tipo_desconto,
            valor_desconto,
            valor_minimo,
            data_validade,
            limite_uso,
            uso_atual: 0,
            status: StatusCupom::Ativo,
            criado_em: chrono::Utc::now(),
        }
    }

    pub fn calcular_desconto(&self, valor_pedido: Decimal, valor_frete: Decimal) -> Decimal {
        match self.tipo_desconto.as_str() {
            "percentual"   => valor_desconto_com_limite(
                valor_pedido * (self.valor_desconto.unwrap_or(Decimal::ZERO) / Decimal::from(100)),
                None,
            ),
            "valor_fixo"   => self.valor_desconto.unwrap_or(Decimal::ZERO),
            "frete_gratis" => valor_frete,
            _              => Decimal::ZERO,
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

impl ToProto<crate::proto::Cupom> for Cupom {
    fn to_proto(&self) -> crate::proto::Cupom {
        crate::proto::Cupom {
            uuid: self.uuid.to_string(),
            loja_uuid: self.loja_uuid.to_string(),
            codigo: self.codigo.clone(),
            descricao: self.descricao.clone(),
            tipo_desconto: self.tipo_desconto.clone(),
            valor_desconto: self.valor_desconto.map(|d| d.to_string()).unwrap_or_default(),
            valor_minimo: self.valor_minimo.map(|d| d.to_string()).unwrap_or_default(),
            data_validade: self.data_validade.to_rfc3339(),
            limite_uso: self.limite_uso.unwrap_or_default(),
            uso_atual: self.uso_atual,
            status: self.status.as_str().to_string(),
        }
    }
}


fn valor_desconto_com_limite(desconto: Decimal, maximo: Option<Decimal>) -> Decimal {
    if let Some(max) = maximo {
        desconto.min(max)
    } else {
        desconto
    }
}

// --- UsoCupom ---

#[allow(dead_code)]
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UsoCupom {
    pub uuid: Uuid,
    pub cupom_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub pedido_uuid: Uuid,
    pub valor_desconto: Decimal,
    pub usado_em: String,
}

#[allow(dead_code)]
impl UsoCupom {
    pub fn new(
        cupom_uuid: Uuid,
        usuario_uuid: Uuid,
        pedido_uuid: Uuid,
        valor_desconto: Decimal,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            cupom_uuid,
            usuario_uuid,
            pedido_uuid,
            valor_desconto,
            usado_em: Utc::now().to_rfc3339().to_owned(),
        }
    }
}


#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Promocao {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub descricao: String,
    pub tipo_desconto: String,
    pub valor_desconto: Option<Decimal>,
    pub valor_minimo: Option<Decimal>,
    pub data_inicio: chrono::DateTime<chrono::Utc>,
    pub data_fim: chrono::DateTime<chrono::Utc>,
    pub dias_semana_validos: Option<Vec<i32>>,
    pub tipo_escopo: String,
    pub produto_uuid: Option<Uuid>,
    pub categoria_uuid: Option<Uuid>,
    pub status: StatusCupom,
    pub prioridade: i32,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
impl Promocao {
    pub fn new(
        loja_uuid: Uuid,
        nome: String,
        descricao: String,
        tipo_desconto: String,
        valor_desconto: Option<Decimal>,
        valor_minimo: Option<Decimal>,
        data_inicio: String,
        data_fim: String,
        dias_semana_validos: Option<Vec<u8>>,
        tipo_escopo: String,
        produto_uuid: Option<Uuid>,
        categoria_uuid: Option<Uuid>,
        prioridade: i32,
    ) -> Self {
        // Parse datetime strings to DateTime<Utc>
        let data_inicio = chrono::DateTime::parse_from_rfc3339(&data_inicio)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());
        let data_fim = chrono::DateTime::parse_from_rfc3339(&data_fim)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());
        
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
            dias_semana_validos: dias_semana_validos.map(|d| d.iter().map(|&n| n as i32).collect()),
            tipo_escopo,
            produto_uuid,
            categoria_uuid,
            status: StatusCupom::Ativo,
            prioridade,
            criado_em: Utc::now()
        }
    }

    /// Verifica se a promoção é aplicável a um determinado produto/categoria
    pub fn eh_aplicavel_a_entidade(&self, produto_uuid: Option<Uuid>, categoria_uuid: Option<Uuid>) -> bool {
        match self.tipo_escopo.as_str() {
            "loja" => true,
            "produto" => self.produto_uuid == produto_uuid,
            "categoria" => self.categoria_uuid == categoria_uuid,
            _ => false,
        }
    }

    pub fn eh_aplicavel(&self, valor_pedido: Decimal, data_hora: chrono::DateTime<chrono::Utc>, dia_semana: u8) -> bool {
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
        if let Some(ref dias) = self.dias_semana_validos {
            let dia_atual = dia_semana as i32;
            if !dias.contains(&dia_atual) {
                return false;
            }
        }
        true
    }

    pub fn calcular_desconto(&self, valor_pedido: Decimal, valor_frete: Decimal) -> Decimal {
        match self.tipo_desconto.as_str() {
            "percentual"   => valor_pedido * (self.valor_desconto.unwrap_or(Decimal::ZERO) / Decimal::from(100)),
            "valor_fixo"   => self.valor_desconto.unwrap_or(Decimal::ZERO),
            "frete_gratis" => valor_frete,
            _              => Decimal::ZERO,
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

impl ToProto<crate::proto::Promocao> for Promocao {
    fn to_proto(&self) -> crate::proto::Promocao {
        crate::proto::Promocao {
            uuid: self.uuid.to_string(),
            loja_uuid: self.loja_uuid.to_string(),
            nome: self.nome.clone(),
            descricao: self.descricao.clone(),
            tipo_desconto: self.tipo_desconto.clone(),
            valor_desconto: self.valor_desconto.map(|d| d.to_string()).unwrap_or_default(),
            status: self.status.as_str().to_string(),
        }
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
