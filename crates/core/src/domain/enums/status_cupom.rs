/// Pure domain enum — SQL encoding handled by repository adapter.

#[derive(Debug, Clone, PartialEq)]
pub enum StatusCupom {
    Ativo,
    Inativo,
    Expirado,
    Esgotado,
}

impl StatusCupom {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ativo => "ativo",
            Self::Inativo => "inativo",
            Self::Expirado => "expirado",
            Self::Esgotado => "esgotado",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "ativo" => Ok(Self::Ativo),
            "inativo" => Ok(Self::Inativo),
            "expirado" => Ok(Self::Expirado),
            "esgotado" => Ok(Self::Esgotado),
            other => Err(format!("Status invalido: {}", other)),
        }
    }
}

impl std::fmt::Display for StatusCupom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
