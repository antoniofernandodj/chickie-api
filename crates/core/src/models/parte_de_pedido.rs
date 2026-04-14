// Reexportar tipos de pedido.rs para evitar duplicação
// Apenas ParteDeItemPedido é usado externamente
pub use crate::models::pedido::ParteDeItemPedido;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::Utc;
use crate::{models::Model, ports::to_proto::ToProto};
use rust_decimal::Decimal;
use utoipa::ToSchema;

// ---------------------------------------------------------------------------
// TipoCalculoPedido — como o preço é calculado quando há múltiplos sabores
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum TipoCalculoPedido {
    MediaPonderada, // média simples: soma dos preços / qtd de sabores
    MaisCaro,       // preço = o sabor mais caro
}

impl TipoCalculoPedido {
    pub fn as_str(&self) -> &str {
        match self {
            Self::MediaPonderada => "media_ponderada",
            Self::MaisCaro       => "mais_caro",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "media_ponderada" => Ok(Self::MediaPonderada),
            "mais_caro"       => Ok(Self::MaisCaro),
            other => Err(format!("TipoCalculoSabor inválido: '{}'", other)),
        }
    }
}

impl std::fmt::Display for TipoCalculoPedido {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for TipoCalculoPedido {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Self::from_str(&s).map_err(|e| e.into())
    }
}

impl sqlx::Type<sqlx::Postgres> for TipoCalculoPedido {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for TipoCalculoPedido {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        <&str as sqlx::Encode<sqlx::Postgres>>::encode(self.as_str(), buf)
    }

    fn produces(&self) -> Option<sqlx::postgres::PgTypeInfo> {
        Some(sqlx::postgres::PgTypeInfo::with_name("TEXT"))
    }
}

// ---------------------------------------------------------------------------
// ConfiguracaoSaboresLoja — configuração POR LOJA
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ConfiguracaoDePedidosLoja {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub max_partes: i32,
    pub tipo_calculo: TipoCalculoPedido,
    pub criado_em: chrono::DateTime<chrono::Utc>,
    pub atualizado_em: chrono::DateTime<chrono::Utc>,
}

impl ConfiguracaoDePedidosLoja {
    pub fn new(
        loja_uuid: Uuid,
        max_partes: i32,
        tipo_calculo: TipoCalculoPedido,
    ) -> Result<Self, String> {
        if max_partes < 1 {
            return Err("max_partes deve ser >= 1".into());
        }
        Ok(Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            max_partes,
            tipo_calculo,
            criado_em: Utc::now(),
            atualizado_em: Utc::now(),
        })
    }

}

impl ToProto<crate::proto::ConfigPedido> for ConfiguracaoDePedidosLoja {
    fn to_proto(&self) -> crate::proto::ConfigPedido {
        crate::proto::ConfigPedido {
            uuid: self.uuid.to_string(),
            loja_uuid: self.loja_uuid.to_string(),
            max_partes: self.max_partes,
            tipo_calculo: self.tipo_calculo.as_str().to_string(),
            criado_em: self.criado_em.to_rfc3339(),
            atualizado_em: self.atualizado_em.to_rfc3339(),
        }
    }
}


// ---------------------------------------------------------------------------
// Lógica de cálculo — funções puras
// ---------------------------------------------------------------------------

/// Calcula o preço unitário de um item com múltiplos sabores.
pub fn calcular_preco_por_partes(
    sabores: &[ParteDeItemPedido],
    tipo: &TipoCalculoPedido,
) -> Decimal {
    if sabores.is_empty() {
        return Decimal::ZERO;
    }

    match tipo {
        &TipoCalculoPedido::MediaPonderada => {
            let soma: Decimal = sabores.iter().map(|s| s.preco_unitario).sum();
            soma / Decimal::from(sabores.len())
        }
        &TipoCalculoPedido::MaisCaro => {
            sabores
                .iter()
                .map(|s| s.preco_unitario)
                .fold(Decimal::MIN, Decimal::max)
        }
    }
}

impl Model for ConfiguracaoDePedidosLoja {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
