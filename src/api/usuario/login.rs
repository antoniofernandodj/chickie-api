use axum::{Json, extract::State, response::IntoResponse};
use serde_json::json;
use std::sync::Arc;
use crate::api::{AppState, create_jwt, dto::{AppError, LoginRequest}};

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Validar as credenciais através do Service
    // O service deve buscar o usuário e comparar o hash da senha
    let usuario = state.usuario_service
        .autenticar(payload.email, payload.senha)
        .await
        .map_err(|_| AppError::BadRequest("Email ou senha inválidos".to_string()))?;

    // 2. Gerar o token JWT usando a função que criamos
    let token = create_jwt(usuario)
        .map_err(|e| AppError::Internal(format!("Erro ao gerar token: {}", e)))?;

    // 3. Retornar o token e opcionalmente dados básicos do usuário
    Ok(Json(json!({
        "access_token": token,
        "token_type": "Bearer"
    })))
}