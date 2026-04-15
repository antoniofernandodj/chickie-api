use axum::extract::State;
use std::sync::Arc;
use crate::handlers::{AppState, create_jwt, dto::AppError, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;
use chickie_core::proto;

pub async fn login(
    State(state): State<Arc<AppState>>,
    Protobuf(payload): Protobuf<proto::LoginRequest>,
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
