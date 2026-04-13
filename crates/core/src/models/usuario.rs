use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::Utc;
use utoipa::ToSchema;

use crate::models::Model;

// ===========================================================================
// ClasseUsuario — define o papel do usuário no sistema
// ===========================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Usuario {
    pub nome: String,
    pub username: String,
    pub email: String,
    pub celular: String,
    pub criado_em: chrono::DateTime<chrono::Utc>,
    pub atualizado_em: chrono::DateTime<chrono::Utc>,

    pub modo_de_cadastro: String,
    pub classe: String,  // "cliente" | "administrador"

    pub senha_hash: String,
    pub uuid: Uuid,
    pub ativo: bool,
    pub passou_pelo_primeiro_acesso: bool,

    // Soft delete fields
    #[serde(default)]
    pub marcado_para_remocao: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub deletado: bool,

    // Block flag — explicitly blocks user from logging in
    #[serde(default)]
    pub bloqueado: bool,
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
            criado_em: Utc::now(),
            atualizado_em: Utc::now(),
            modo_de_cadastro,
            classe: classe.as_str().to_string(),

            senha_hash,
            uuid: Uuid::new_v4(),
            ativo: true,
            passou_pelo_primeiro_acesso: false,
            marcado_para_remocao: None,
            deletado: false,
            bloqueado: false,
        }
    }

    /// Verifica se este usuário é um administrador
    pub fn is_administrador(&self) -> bool {
        self.classe == ClasseUsuario::Administrador.as_str()
    }

    /// Verifica se este usuário é o dono da plataforma (owner)
    pub fn is_owner(&self) -> bool {
        self.classe == ClasseUsuario::Owner.as_str()
    }

    /// Verifica se o usuário está marcado para remoção
    pub fn esta_marcado_para_remocao(&self) -> bool {
        self.marcado_para_remocao.is_some() && !self.deletado
    }

    /// Verifica se o usuário está permanentemente deletado
    pub fn esta_deletado(&self) -> bool {
        self.deletado
    }

    /// Verifica se o usuário está ativo (não deletado e não marcado para remoção)
    pub fn esta_ativo_para_login(&self) -> bool {
        !self.deletado && self.marcado_para_remocao.is_none() && self.ativo && !self.bloqueado
    }

    /// Verifica se o usuário está bloqueado
    pub fn esta_bloqueado(&self) -> bool {
        self.bloqueado
    }
}

impl Model for Usuario {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
