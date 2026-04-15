/// Pure domain enum — no sqlx, no serde, no utoipa.
/// SQL encoding is handled by the repository adapter layer.

#[derive(Debug, Clone, PartialEq)]
pub enum ClasseUsuario {
    Cliente,
    Administrador,
    Funcionario,
    Entregador,
    Owner,
}

impl ClasseUsuario {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Cliente => "cliente",
            Self::Administrador => "administrador",
            Self::Funcionario => "funcionario",
            Self::Entregador => "entregador",
            Self::Owner => "owner",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "cliente" => Ok(Self::Cliente),
            "administrador" => Ok(Self::Administrador),
            "funcionario" => Ok(Self::Funcionario),
            "entregador" => Ok(Self::Entregador),
            "owner" => Ok(Self::Owner),
            other => Err(format!("ClasseUsuario invalida: '{}'", other)),
        }
    }
}

impl std::fmt::Display for ClasseUsuario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
