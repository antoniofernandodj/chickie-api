use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::utils::agora;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Usuario {
    pub nome: String,
    pub username: String,
    pub email: String,
    pub celular: String,
    pub criado_em: String,
    pub atualizado_em: String,
    
    pub modo_de_cadastro: String,
    
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
    ) -> Self {

        Self {
            nome,
            username,
            email,
            celular,
            criado_em: agora(),
            atualizado_em: agora(),
            modo_de_cadastro,

            telefone: None,
            senha_hash,
            uuid: Uuid::new_v4(),
            ativo: true,
            passou_pelo_primeiro_acesso: false,
        }
    }
}
