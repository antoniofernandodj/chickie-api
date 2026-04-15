/// Pure domain enum — SQL encoding handled by repository adapter.

#[derive(Debug, Clone, PartialEq)]
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
            other => Err(format!("Escopo invalido: '{}'", other)),
        }
    }
}

impl std::fmt::Display for TipoEscopoPromocao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
