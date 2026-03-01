use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Deserialize)]
pub struct CreateUsuarioRequest {
    pub nome: String,
    pub username: String,
    pub senha: String,
    pub email: String,
    pub telefone: String,
    pub auth_method: String,
}

#[derive(Deserialize)]
pub struct CreateLojaRequest {
    pub nome: String,
    pub slug: String,
    pub email_contato: String,
    pub descricao: Option<String>,
    pub telefone: Option<String>,
    pub hora_abertura: Option<String>,
    pub hora_fechamento: Option<String>,
    pub dias_funcionamento: Option<String>,
    pub tempo_medio: Option<i32>,
    pub nota_media: Option<f64>,
    pub taxa_entrega_base: Option<f64>,
    pub pedido_minimo: Option<f64>,
    pub max_partes: i32,
}

#[derive(Deserialize)]
pub struct CreatePedidoRequest {
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub taxa_entrega: f64,
    pub forma_pagamento: String,
    pub observacoes: Option<String>,
    pub codigo_cupom: Option<String>,
    pub itens: Vec<ItemPedidoRequest>,
}

#[derive(Deserialize)]
pub struct ItemPedidoRequest {
    pub quantidade: i32,
    pub observacoes: Option<String>,
    pub partes: Vec<ParteItemRequest>,
}

#[derive(Deserialize)]
pub struct ParteItemRequest {
    pub produto_uuid: Uuid,
    pub posicao: i32,
}

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum AppError {
    NotFound(String),
    Internal(String),
    BadRequest(String)
    // Adicione outros como BadRequest, Unauthorized, etc.
}

// O "pulo do gato": transforma seu erro em uma resposta do Axum automaticamente
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}

// Permite que o erro do banco (sqlx, etc) vire um AppError com o operador '?'
// Implementação para String
impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::BadRequest(error)
    }
}

// Implementação para &str (literais como "Erro aqui")
impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError::BadRequest(error.to_string())
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,    // Geralmente o UUID do usuário em formato String
    pub exp: usize,     // Timestamp de expiração
    pub iat: usize,     // Timestamp de quando foi emitido (opcional)
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub senha: String,
}

