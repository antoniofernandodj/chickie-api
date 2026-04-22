use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::Model;
use chrono::Utc;
use sqlx::FromRow;
use rust_decimal::Decimal;
use utoipa::ToSchema;

// --- AdicionalDeItemDePedido (agora dentro do JSONB) ---

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AdicionalDeItemDePedido {
    pub uuid: Uuid,
    pub item_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub descricao: String,
    pub preco: Decimal,
}

impl AdicionalDeItemDePedido {
    pub fn new(
        nome: String,
        descricao: String,
        loja_uuid: Uuid,
        item_uuid: Uuid,
        preco: Decimal,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            item_uuid,
            loja_uuid,
            nome,
            descricao,
            preco,
        }
    }
}

// --- ParteDeItemPedido (agora dentro do JSONB) ---

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ParteDeItemPedido {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub item_uuid: Option<Uuid>,
    pub produto_nome: String,
    pub produto_uuid: Uuid,
    pub preco_unitario: Decimal,
    pub posicao: i32,
    #[serde(default)]
    pub adicionais: Vec<AdicionalDeItemDePedido>,
}

impl ParteDeItemPedido {
    pub fn new(
        produto_uuid: Uuid,
        produto_nome: String,
        loja_uuid: Uuid,
        preco_unitario: Decimal,
        posicao: i32,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            item_uuid: None,
            produto_nome,
            produto_uuid,
            preco_unitario,
            posicao,
            adicionais: vec![],
        }
    }
}

// --- ItemPedido (agora dentro do JSONB) ---

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ItemPedido {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub pedido_uuid: Uuid,
    pub quantidade: i32,
    pub observacoes: Option<String>,
    #[serde(default)]
    pub partes: Vec<ParteDeItemPedido>,
    #[serde(default)]
    pub adicionais: Vec<AdicionalDeItemDePedido>,
}

impl ItemPedido {
    pub fn new(
        pedido_uuid: Uuid,
        loja_uuid: Uuid,
        quantidade: i32,
        observacoes: Option<String>,
        partes: Vec<ParteDeItemPedido>
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            pedido_uuid,
            loja_uuid,
            quantidade,
            observacoes,
            partes,
            adicionais: Vec::new(),
        }
    }
}

// --- EstadoDePedido ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ToSchema)]
pub enum EstadoDePedido {
    Criado,
    AguardandoConfirmacaoDeLoja,
    ConfirmadoPelaLoja,
    EmPreparo,
    ProntoParaRetirada,
    SaiuParaEntrega,
    Entregue,
}

impl std::fmt::Display for EstadoDePedido {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[allow(dead_code)]
impl EstadoDePedido {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Criado                      => "criado",
            Self::AguardandoConfirmacaoDeLoja => "aguardando_confirmacao_de_loja",
            Self::ConfirmadoPelaLoja          => "confirmado_pela_loja",
            Self::EmPreparo                   => "em_preparo",
            Self::ProntoParaRetirada          => "pronto_para_retirada",
            Self::SaiuParaEntrega             => "saiu_para_entrega",
            Self::Entregue                    => "entregue",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "criado"                         => Ok(Self::Criado),
            "aguardando_confirmacao_de_loja" => Ok(Self::AguardandoConfirmacaoDeLoja),
            "confirmado_pela_loja"           => Ok(Self::ConfirmadoPelaLoja),
            "em_preparo"                     => Ok(Self::EmPreparo),
            "pronto_para_retirada"           => Ok(Self::ProntoParaRetirada),
            "saiu_para_entrega"              => Ok(Self::SaiuParaEntrega),
            "entregue"                       => Ok(Self::Entregue),
            other => Err(format!("Estado inválido: {}", other)),
        }
    }

    /// Retorna o próximo estado válido a partir do estado atual.
    /// Retorna `Err` se o estado já for terminal ou se não houver transição.
    pub fn avancar(&self) -> Result<Self, String> {
        match self {
            Self::Criado => Ok(Self::AguardandoConfirmacaoDeLoja),
            Self::AguardandoConfirmacaoDeLoja => Ok(Self::ConfirmadoPelaLoja),
            Self::ConfirmadoPelaLoja => Ok(Self::EmPreparo),
            Self::EmPreparo => Ok(Self::ProntoParaRetirada),
            Self::ProntoParaRetirada => Ok(Self::SaiuParaEntrega),
            Self::SaiuParaEntrega => Ok(Self::Entregue),
            Self::Entregue => Err("Pedido já foi entregue — estado terminal".to_string()),
        }
    }

    /// Transições permitidas para um estado (incluindo avançar e retrocesso controlado)
    pub fn transicoes_permitidas(&self) -> Vec<Self> {
        match self {
            Self::Criado => vec![Self::AguardandoConfirmacaoDeLoja],
            Self::AguardandoConfirmacaoDeLoja => vec![Self::ConfirmadoPelaLoja, Self::Criado],
            Self::ConfirmadoPelaLoja => vec![Self::EmPreparo, Self::AguardandoConfirmacaoDeLoja],
            Self::EmPreparo => vec![Self::ProntoParaRetirada, Self::ConfirmadoPelaLoja],
            Self::ProntoParaRetirada => vec![Self::SaiuParaEntrega, Self::EmPreparo],
            Self::SaiuParaEntrega => vec![Self::Entregue, Self::ProntoParaRetirada],
            Self::Entregue => vec![],
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Entregue)
    }

    /// Verifica se a transição para `proximo` é válida
    pub fn pode_transicionar_para(&self, proximo: &Self) -> bool {
        self.transicoes_permitidas().contains(proximo)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for EstadoDePedido {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>
    ) -> Result<Self, sqlx::error::BoxDynError> {
        // PostgreSQL retorna &str para TEXT
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Self::from_str(s).map_err(|e| e.into())
    }
}

impl sqlx::Type<sqlx::Postgres> for EstadoDePedido {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        // Mapeia para TEXT no PostgreSQL
        sqlx::postgres::PgTypeInfo::with_name("TEXT")
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for EstadoDePedido {
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
// --- Pedido ---

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Pedido {
    pub uuid: Uuid,
    pub codigo: String,
    pub usuario_uuid: Option<Uuid>,
    pub loja_uuid: Uuid,
    pub entregador_uuid: Option<Uuid>,
    pub status: EstadoDePedido,
    pub total: Decimal,
    pub subtotal: Decimal,
    pub taxa_entrega: Decimal,
    pub desconto: Option<Decimal>,
    pub forma_pagamento: String,
    pub observacoes: Option<String>,
    pub tempo_estimado_min: Option<i32>,
    pub criado_em: chrono::DateTime<chrono::Utc>,
    pub atualizado_em: chrono::DateTime<chrono::Utc>,
    /// Coluna JSONB do banco — mapeada pelo sqlx, parseada para `itens`
    #[sqlx(rename = "itens")]
    #[serde(skip)]
    pub itens_json: serde_json::Value,
    /// Itens parseados a partir de `itens_json` (não mapeado pelo sqlx)
    #[sqlx(skip)]
    #[serde(default)]
    pub itens: Vec<ItemPedido>,
}

#[allow(dead_code)]
impl Pedido {
    pub fn new(
        usuario_uuid: Option<Uuid>,
        loja_uuid: Uuid,
        subtotal: Decimal,
        taxa_entrega: Decimal,
        forma_pagamento: String,
        observacoes: Option<String>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            codigo: String::new(),
            usuario_uuid,
            loja_uuid,
            entregador_uuid: None,
            status: EstadoDePedido::Criado,
            total: subtotal + taxa_entrega,
            subtotal,
            taxa_entrega,
            desconto: None,
            forma_pagamento,
            observacoes,
            tempo_estimado_min: None,
            criado_em: Utc::now(),
            atualizado_em: Utc::now(),
            itens_json: serde_json::Value::Array(vec![]),
            itens: Vec::new(),
        }
    }

    pub fn adicionar_item<'a>(
        &'a mut self,
        quantidade: i32,
        observacoes: Option<String>,
        partes: Vec<ParteDeItemPedido>
    ) -> Uuid {

        let mut item: ItemPedido = ItemPedido::new(
            self.uuid,
            self.loja_uuid,
            quantidade,
            observacoes,
            partes,
        );
        
        let item_uuid = item.uuid;
        for parte in item.partes.iter_mut() {
            parte.item_uuid = Some(item.uuid);
        };

        self.itens.push(item);

        return item_uuid;
    }

    pub fn localizar_item(
        &mut self, item_uuid: Uuid
    ) -> &mut ItemPedido {
        self
            .itens
            .iter_mut()
            .find(|i| i.uuid == item_uuid)
            .expect("Item não encontrado")
    }
}

impl Model for Pedido {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
