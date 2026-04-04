use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{models::Model, utils::agora};

// ===========================================================================
// ClasseUsuario — define o papel do usuário no sistema
// ===========================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClasseUsuario {
    Cliente,
    Administrador,
    Funcionario,
    Entregador,
}

impl ClasseUsuario {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Cliente => "cliente",
            Self::Administrador => "administrador",
            Self::Funcionario => "funcionario",
            Self::Entregador => "entregador",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "cliente" => Ok(Self::Cliente),
            "administrador" => Ok(Self::Administrador),
            "funcionario" => Ok(Self::Funcionario),
            "entregador" => Ok(Self::Entregador),
            other => Err(format!("ClasseUsuario inválida: '{}'", other)),
        }
    }
}

impl std::fmt::Display for ClasseUsuario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ===========================================================================
// Usuario
// ===========================================================================

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Usuario {
    pub nome: String,
    pub username: String,
    pub email: String,
    pub celular: String,
    pub criado_em: String,
    pub atualizado_em: String,

    pub modo_de_cadastro: String,
    pub classe: String,  // "cliente" | "administrador"

    pub telefone: Option<String>,
    pub senha_hash: String,
    pub uuid: Uuid,
    pub ativo: bool,
    pub passou_pelo_primeiro_acesso: bool
}

impl Usuario {
    pub fn new(
        nome: String,
        username: String,
        email: String,
        senha_hash: String,
        celular: String,
        modo_de_cadastro: String,
        classe: ClasseUsuario,
    ) -> Self {

        Self {
            nome,
            username,
            email,
            celular,
            criado_em: agora(),
            atualizado_em: agora(),
            modo_de_cadastro,
            classe: classe.as_str().to_string(),

            telefone: None,
            senha_hash,
            uuid: Uuid::new_v4(),
            ativo: true,
            passou_pelo_primeiro_acesso: false,
        }
    }

    /// Verifica se este usuário é um administrador
    pub fn is_administrador(&self) -> bool {
        self.classe == ClasseUsuario::Administrador.as_str()
    }
}

impl Model for Usuario {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
