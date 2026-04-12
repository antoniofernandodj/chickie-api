use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::{IntoResponses, ToSchema};


#[derive(Deserialize, ToSchema)]
pub struct CreateUsuarioRequest {
    pub nome: String,
    pub username: String,
    pub senha: String,
    pub email: String,
    pub celular: String,
    pub auth_method: String,
    #[schema(example = "cliente")]
    pub classe: Option<String>,  // "cliente" (default) | "administrador"
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateLojaRequest {
    pub nome: String,
    pub slug: String,
    pub email_contato: String,
    pub descricao: Option<String>,
    pub celular: Option<String>,
    pub hora_abertura: Option<String>,
    pub hora_fechamento: Option<String>,
    pub dias_funcionamento: Option<Vec<i32>>,  // [0,1,2,3,4,5,6]
    pub tempo_medio: Option<i32>,
    pub nota_media: Option<f64>,
    pub taxa_entrega_base: Option<f64>,
    pub pedido_minimo: Option<f64>,
    pub max_partes: i32,
}

#[allow(dead_code)]
#[derive(Deserialize, ToSchema)]
pub struct CreatePedidoRequest {
    // pub loja_uuid: Uuid,
    // pub usuario_uuid: Uuid,
    pub taxa_entrega: f64,
    pub forma_pagamento: String,
    pub observacoes: Option<String>,
    pub codigo_cupom: Option<String>,
    pub itens: Vec<ItemPedidoRequest>,

    // === NOVO: Endereço de entrega para o pedido ===
    pub endereco_entrega: DadosEnderecoEntregaRequest,
}


/// Dados de entrada para o endereço de entrega (snapshot no momento do pedido)
#[allow(dead_code)]
#[derive(Deserialize, Clone, ToSchema)]
pub struct DadosEnderecoEntregaRequest {
    pub cep: Option<String>,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    // pub latitude: Option<f64>,
    // pub longitude: Option<f64>,
}

#[allow(dead_code)]
impl DadosEnderecoEntregaRequest {
    /// Converte para o modelo de domínio EnderecoEntrega
    pub fn to_endereco_entrega(
        self,
        pedido_uuid: Uuid,
        loja_uuid: Uuid,
    ) -> chickie_core::models::EnderecoEntrega {
        chickie_core::models::EnderecoEntrega::new(
            pedido_uuid,
            loja_uuid,
            self.cep,
            self.logradouro,
            self.numero,
            self.complemento,
            self.bairro,
            self.cidade,
            self.estado,
            // self.latitude,
            // self.longitude,
        )
    }
}


#[allow(dead_code)]
#[derive(Deserialize, ToSchema)]
pub struct ItemPedidoRequest {
    pub quantidade: i32,
    pub observacoes: Option<String>,
    pub partes: Vec<ParteItemRequest>,
}

#[allow(dead_code)]
#[derive(Deserialize, ToSchema)]
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
use chickie_core::domain::errors::DomainError;

#[derive(IntoResponses)]
pub enum AppError {
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 500)]
    Internal(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 403)]
    Unauthorized(String),
    #[response(status = 409)]
    Conflict(String),
    #[response(status = 422)]
    InvalidState(String),
}

// O "pulo do gato": transforma seu erro em uma resposta do Axum automaticamente
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::InvalidState(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}

// DomainError → AppError mapping (Clean Architecture boundary)
impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::NotFound { entity, id } => {
                AppError::NotFound(format!("{} not found: {}", entity, id))
            }
            DomainError::BusinessRule(msg) => AppError::BadRequest(msg),
            DomainError::Validation(msg) => AppError::BadRequest(msg),
            DomainError::Conflict { entity, field } => {
                AppError::Conflict(format!("{} conflict on field '{}'", entity, field))
            }
            DomainError::InvalidState { current, attempted, allowed } => {
                AppError::InvalidState(format!(
                    "Invalid state transition: {} -> {}. Allowed: {:?}",
                    current, attempted, allowed
                ))
            }
            DomainError::Internal(msg) => AppError::Internal(msg),
        }
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


#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Claims {
    pub sub: String,    // Geralmente o UUID do usuário em formato String
    pub exp: usize,     // Timestamp de expiração
    pub iat: usize,     // Timestamp de quando foi emitido (opcional)
}

#[derive(serde::Deserialize, ToSchema)]
pub struct LoginRequest {
    /// Email, username ou celular
    pub identifier: String,
    pub senha: String,
}

#[derive(serde::Deserialize, ToSchema)]
pub struct AvaliarLojaRequest {
    pub nota: rust_decimal::Decimal,
    pub comentario: Option<String>,
}

#[derive(serde::Deserialize, ToSchema)]
pub struct AvaliarProdutoRequest {
    pub produto_uuid: uuid::Uuid,
    pub nota: rust_decimal::Decimal,
    pub descricao: String,
    pub comentario: Option<String>,
}

#[derive(serde::Deserialize, ToSchema)]
pub struct AtualizarAvaliacaoLojaRequest {
    pub nota: rust_decimal::Decimal,
    pub comentario: Option<String>,
}

#[derive(serde::Deserialize, ToSchema)]
pub struct AtualizarAvaliacaoProdutoRequest {
    pub nota: rust_decimal::Decimal,
    pub descricao: String,
    pub comentario: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct VerificarEmailRequest {
    pub email: String,
}

#[derive(Deserialize, ToSchema)]
pub struct VerificarUsernameRequest {
    pub username: String,
}

#[derive(Serialize, ToSchema)]
pub struct DisponivelResponse {
    pub disponivel: bool,
}

