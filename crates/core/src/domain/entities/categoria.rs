use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct CategoriaProdutos {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub descricao: Option<String>,
    pub ordem: Option<i32>,
    pub pizza_mode: bool,
    pub criado_em: DateTime<Utc>,
}

impl CategoriaProdutos {
    pub fn new(nome: String, descricao: Option<String>, loja_uuid: Uuid, ordem: Option<i32>, pizza_mode: bool) -> Self {
        Self { uuid: Uuid::new_v4(), loja_uuid, nome, descricao, ordem, pizza_mode, criado_em: Utc::now() }
    }
}
