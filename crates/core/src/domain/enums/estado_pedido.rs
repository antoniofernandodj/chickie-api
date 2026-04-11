/// Pure domain enum — SQL encoding handled by repository adapter.

#[derive(Debug, PartialEq, Clone)]
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
            Self::Criado => "criado",
            Self::AguardandoConfirmacaoDeLoja => "aguardando_confirmacao_de_loja",
            Self::ConfirmadoPelaLoja => "confirmado_pela_loja",
            Self::EmPreparo => "em_preparo",
            Self::ProntoParaRetirada => "pronto_para_retirada",
            Self::SaiuParaEntrega => "saiu_para_entrega",
            Self::Entregue => "entregue",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "criado" => Ok(Self::Criado),
            "aguardando_confirmacao_de_loja" => Ok(Self::AguardandoConfirmacaoDeLoja),
            "confirmado_pela_loja" => Ok(Self::ConfirmadoPelaLoja),
            "em_preparo" => Ok(Self::EmPreparo),
            "pronto_para_retirada" => Ok(Self::ProntoParaRetirada),
            "saiu_para_entrega" => Ok(Self::SaiuParaEntrega),
            "entregue" => Ok(Self::Entregue),
            other => Err(format!("Estado invalid: {}", other)),
        }
    }

    pub fn avancar(&self) -> Result<Self, String> {
        match self {
            Self::Criado => Ok(Self::AguardandoConfirmacaoDeLoja),
            Self::AguardandoConfirmacaoDeLoja => Ok(Self::ConfirmadoPelaLoja),
            Self::ConfirmadoPelaLoja => Ok(Self::EmPreparo),
            Self::EmPreparo => Ok(Self::ProntoParaRetirada),
            Self::ProntoParaRetirada => Ok(Self::SaiuParaEntrega),
            Self::SaiuParaEntrega => Ok(Self::Entregue),
            Self::Entregue => Err("Pedido ja foi entregue -- estado terminal".to_string()),
        }
    }

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

    pub fn pode_transicionar_para(&self, proximo: &Self) -> bool {
        self.transicoes_permitidas().contains(proximo)
    }
}
