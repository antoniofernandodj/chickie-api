use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    http::{StatusCode, header},
    Json,
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde_json::json;
use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use crate::{api::{AppState, Claims}, models::Usuario, repositories::Repository}; // Importe seus Claims e AppState


pub async fn auth_middleware(
    State(state): State<Arc<AppState>>, // Agora o middleware recebe o estado
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    // 1. Extração do Token
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .filter(|h| h.starts_with("Bearer "))
        .map(|h| &h[7..])
        .ok_or_else(|| (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Token de autenticação não fornecido. Inclua o header: Authorization: Bearer <token>" }))
        ))?;

    // 2. Decodificação do JWT
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
    let token_data = decode::<Claims>(
        auth_header,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ).map_err(|e| {
        tracing::warn!("Falha ao decodificar JWT: {:?}", e);
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Token inválido ou expirado. Faça login novamente." }))
        )
    })?;

    // 3. Consulta ao Banco de Dados
    // Usamos o 'sub' (ID do usuário) do token para buscar no repositório
    let user_uuid = token_data.claims.sub.parse().map_err(|_| (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "Token mal formado: UUID do usuário inválido" }))
    ))?;

    let usuario = state.usuario_repo
        .buscar_por_uuid(user_uuid)
        .await
        .map_err(|e| {
            tracing::error!("Erro ao buscar usuário no banco: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Erro interno ao validar usuário" }))
            )
        })?
        .ok_or_else(|| (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Usuário do token não encontrado no banco de dados" }))
        ))?;

    // 4. Injeta o objeto Usuario completo na Request
    req.extensions_mut().insert(usuario);

    Ok(next.run(req).await)
}


pub fn create_jwt(usuario: Usuario) -> Result<String, jsonwebtoken::errors::Error> {
    // 1. Definir o tempo de expiração (ex: 24 horas a partir de agora)
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;
    
    let expires_at = now + (24 * 3600); 

    // 2. Criar os Claims com os dados do usuário
    let claims = Claims {
        sub: usuario.uuid.to_string(), // O UUID vira a identidade do token
        exp: expires_at,
        iat: now,
    };

    // 3. Assinar o token com sua chave secreta
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}