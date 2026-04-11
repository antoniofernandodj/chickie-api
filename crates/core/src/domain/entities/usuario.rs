use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::enums::ClasseUsuario;

#[derive(Debug, Clone, PartialEq)]
pub struct Usuario {
    pub uuid: Uuid,
    pub nome: String,
    pub username: String,
    pub email: String,
    pub celular: String,
    pub senha_hash: String,
    pub ativo: bool,
    pub passou_pelo_primeiro_acesso: bool,
    pub classe: String,
    pub modo_de_cadastro: String,
    pub criado_em: DateTime<Utc>,
    pub atualizado_em: DateTime<Utc>,
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
        let now = Utc::now();
        Self {
            nome,
            username,
            email,
            celular,
            senha_hash,
            ativo: true,
            passou_pelo_primeiro_acesso: false,
            classe: classe.as_str().to_string(),
            modo_de_cadastro,
            uuid: Uuid::new_v4(),
            criado_em: now,
            atualizado_em: now,
        }
    }

    pub fn is_administrador(&self) -> bool {
        self.classe == ClasseUsuario::Administrador.as_str()
    }
}
