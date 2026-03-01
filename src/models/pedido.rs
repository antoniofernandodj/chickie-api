use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{models::{ParteDeItemPedido}, utils::agora};
use sqlx::FromRow;

// --- AdicionalDeItemDePedido ---

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AdicionalDeItemDePedido {
    pub uuid: Uuid,
    pub item_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub descricao: String,
    pub preco: f64,
}

impl AdicionalDeItemDePedido {
    pub fn new(
        nome: String,
        descricao: String,
        loja_uuid: Uuid,
        item_uuid: Uuid,
        preco: f64,
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

// --- ItemPedido ---

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ItemPedido {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub pedido_uuid: Uuid,
    pub quantidade: i32,
    pub observacoes: Option<String>,
    #[sqlx(skip)]
    pub adicionais: Vec<AdicionalDeItemDePedido>,
    #[sqlx(skip)]
    pub partes: Vec<ParteDeItemPedido>,
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
            adicionais: Vec::new(),
            partes,
        }
    }
}

// --- EstadoDePedido ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
}

impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for EstadoDePedido {
    fn decode(
        value: sqlx::sqlite::SqliteValueRef<'r>
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
        Self::from_str(&s).map_err(|e| e.into())
    }
}

impl sqlx::Type<sqlx::Sqlite> for EstadoDePedido {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}

impl sqlx::Encode<'_, sqlx::Sqlite> for EstadoDePedido {
    fn encode_by_ref(
        &self, args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'_>>
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        args.push(sqlx::sqlite::SqliteArgumentValue::Text(
            self.as_str().to_string().into()
        ));
        Ok(sqlx::encode::IsNull::No)
    }
}

// --- Pedido ---

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Pedido {
    pub uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub status: EstadoDePedido,
    pub total: f64,
    pub subtotal: f64,
    pub taxa_entrega: f64,
    pub desconto: Option<f64>,
    pub forma_pagamento: String,
    pub observacoes: Option<String>,
    pub tempo_estimado_min: Option<i32>,
    pub criado_em: String,
    pub atualizado_em: String,
    #[sqlx(skip)]
    pub itens: Vec<ItemPedido>,
    #[sqlx(skip)]
    pub partes: Vec<ParteDeItemPedido>
}

impl Pedido {
    pub fn new(
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
        subtotal: f64,
        taxa_entrega: f64,
        forma_pagamento: String,
        observacoes: Option<String>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            usuario_uuid,
            loja_uuid,
            status: EstadoDePedido::Criado,
            total: subtotal + taxa_entrega,
            subtotal,
            taxa_entrega,
            desconto: None,
            forma_pagamento,
            observacoes,
            tempo_estimado_min: None,
            criado_em: agora(),
            atualizado_em: agora(),
            itens: Vec::new(),
            partes: Vec::new()
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
