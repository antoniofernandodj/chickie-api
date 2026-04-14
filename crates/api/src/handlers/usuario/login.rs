use axum::{extract::State, Json};
use std::sync::Arc;
use crate::handlers::{AppState, create_jwt, dto::{AppError, LoginRequest}, protobuf::Protobuf};
use chickie_core::proto;

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Protobuf<proto::LoginResponse>, AppError> {
    // 1. Validar as credenciais através do Service
    let mut usuario = state.usuario_service
        .autenticar(payload.identifier, payload.senha)
        .await
        .map_err(|_| AppError::BadRequest("Credenciais inválidas".to_string()))?;

    // 2. Se o email corresponde ao OWNER_EMAIL, sobrescrever a classe para "owner"
    let owner_email = std::env::var("OWNER_EMAIL").unwrap_or_default();
    if !owner_email.is_empty() && usuario.email == owner_email {
        usuario.classe = "owner".to_string();
    }

    // 3. Gerar o token JWT com os dados atualizados do usuário
    let token = create_jwt(usuario.clone())
        .map_err(|e| AppError::Internal(format!("Erro ao gerar token: {}", e)))?;

    // 4. Retornar via Protobuf
    Ok(Protobuf(proto::LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        usuario: Some(usuario.to_proto()),
    }))
}

/*

use axum::{Json, extract::State, response::IntoResponse};
use serde_json::json;
use std::sync::Arc;
use crate::handlers::{AppState, create_jwt, dto::{AppError, LoginRequest}};

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Validar as credenciais através do Service
    // O service busca o usuário por email, username ou celular e compara a senha
    let mut usuario = state.usuario_service
        .autenticar(payload.identifier, payload.senha)
        .await
        .map_err(|_| AppError::BadRequest("Credenciais inválidas".to_string()))?;

    // 2. Se o email corresponde ao OWNER_EMAIL, sobrescrever a classe para "owner"
    let owner_email = std::env::var("OWNER_EMAIL").unwrap_or_default();
    if !owner_email.is_empty() && usuario.email == owner_email {
        usuario.classe = "owner".to_string();
    }

    // 3. Gerar o token JWT com os dados atualizados do usuário
    let token = create_jwt(usuario.clone())
        .map_err(|e| AppError::Internal(format!("Erro ao gerar token: {}", e)))?;

    // 4. Retornar o token e dados básicos do usuário (com classe correta)
    Ok(Json(json!({
        "access_token": token,
        "token_type": "Bearer",
        "usuario": {
            "uuid": usuario.uuid,
            "nome": usuario.nome,
            "username": usuario.username,
            "email": usuario.email,
            "classe": usuario.classe,
            "ativo": usuario.ativo,
            "bloqueado": usuario.bloqueado,
        }
    })))
}

*/