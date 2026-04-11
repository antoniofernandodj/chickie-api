/// Pure domain enum — SQL encoding handled by repository adapter.

#[derive(Debug, Clone, PartialEq)]
pub enum TipoCalculoPedido {
    MediaPonderada,
    MaisCaro,
}

impl TipoCalculoPedido {
    pub fn as_str(&self) -> &str {
        match self {
            Self::MediaPonderada => "media_ponderada",
            Self::MaisCaro => "mais_caro",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "media_ponderada" => Ok(Self::MediaPonderada),
            "mais_caro" => Ok(Self::MaisCaro),
            other => Err(format!("TipoCalculoSabor invalido: '{}'", other)),
        }
    }
}

impl std::fmt::Display for TipoCalculoPedido {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
