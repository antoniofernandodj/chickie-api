use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use chrono::Utc;
use crate::models::{Adicional, AdicionalDeItemDePedido, Model, Produto};
use sqlx::PgPool;
use rust_decimal::Decimal;

// ---------------------------------------------------------------------------
// TipoCalculoPedido — como o preço é calculado quando há múltiplos sabores
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
        // ✅ Encode como TEXT (string) no PostgreSQL
        <&str as sqlx::Encode<sqlx::Postgres>>::encode(self.as_str(), buf)
    }

    fn produces(&self) -> Option<sqlx::postgres::PgTypeInfo> {
        Some(sqlx::postgres::PgTypeInfo::with_name("TEXT"))
    }
}

// ---------------------------------------------------------------------------
// ConfiguracaoSaboresLoja — configuração POR LOJA
// Define quantos sabores são permitidos por padrão e qual cálculo usar.
// Uma loja pode ter apenas uma configuração ativa.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ConfiguracaoDePedidosLoja {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub max_partes: i32,               // máximo de sabores por item (ex: 4)
    pub tipo_calculo: TipoCalculoPedido, // como calcular o preço final
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

// ---------------------------------------------------------------------------
// ParteDeItemPedido — snapshot dos partes escolhidos em um item de pedido
// Cada linha = 1 parte escolhida pelo cliente para aquele item.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ParteDeItemPedido {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub item_uuid: Option<Uuid>,
    pub produto_nome: String,
    pub produto_uuid: Uuid,
    pub preco_unitario: Decimal,
    pub posicao: i32,
    #[sqlx(skip)]
    pub adicionais: Vec<AdicionalDeItemDePedido>
}

impl ParteDeItemPedido {
    pub fn new(
        produto: &Produto,
        posicao: i32,
    ) -> Self {

        Self {
            uuid: Uuid::new_v4(),
            loja_uuid: produto.loja_uuid,
            item_uuid: None,
            produto_uuid: produto.uuid,
            produto_nome: produto.nome.clone(),
            preco_unitario: produto.preco,
            posicao,
            adicionais: vec![]
        }
    }

    pub fn set_item_uuid(&mut self, item_uuid: Uuid) {
        self.item_uuid = Some(item_uuid)
    }

    pub fn adicionar_adicional(
        &mut self,
        adicional: &Adicional
    ) -> Result<Uuid, String> {
        if self.loja_uuid != adicional.loja_uuid {
            return Err("Adicional pertence a outra loja".to_string());
        }

        let novo = AdicionalDeItemDePedido::new(
            adicional.nome.clone(),
            adicional.descricao.clone(),
            adicional.loja_uuid,
            self.uuid,
            adicional.preco,
        );

        let uuid = novo.uuid;
        self.adicionais.push(novo);
        Ok(uuid)
    }
}

// ---------------------------------------------------------------------------
// Lógica de cálculo — funções puras, sem banco de dados
// ---------------------------------------------------------------------------

/// Calcula o preço unitário de um item com múltiplos sabores.
/// Retorna Decimal::ZERO se não houver sabores.
pub fn calcular_preco_por_partes(
    sabores: &[ParteDeItemPedido],
    tipo: &TipoCalculoPedido,
) -> Decimal {
    if sabores.is_empty() {
        return Decimal::ZERO;
    }

    match tipo {
        // Cada sabor contribui igualmente: média simples dos preços
        // Ex: [30.0, 40.0, 35.0, 50.0] → (30+40+35+50) / 4 = 38.75
        &TipoCalculoPedido::MediaPonderada => {
            let soma: Decimal = sabores.iter().map(|s| s.preco_unitario).sum();
            soma / Decimal::from(sabores.len())
        }

        // Preço = o sabor mais caro
        // Ex: [30.0, 40.0, 35.0, 50.0] → 50.0
        &TipoCalculoPedido::MaisCaro => {
            sabores
                .iter()
                .map(|s| s.preco_unitario)
                .fold(Decimal::MIN, Decimal::max)
        }
    }
}

impl Model for ParteDeItemPedido {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}


impl Model for ConfiguracaoDePedidosLoja {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}