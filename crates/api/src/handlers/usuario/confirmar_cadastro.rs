use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use std::{collections::HashMap, sync::Arc};
use serde_json::json;
use crate::handlers::{AppState, create_jwt, dto::AppError};

pub async fn confirmar_cadastro(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let token = params.get("token")
        .ok_or_else(|| AppError::BadRequest("Parâmetro 'token' não informado.".into()))?;

    let usuario = state.usuario_service
        .confirmar_cadastro(token)
        .await?;

    let jwt = create_jwt(usuario.clone())
        .map_err(|e| AppError::Internal(format!("Erro ao gerar token JWT: {}", e)))?;

    Ok(Json(json!({
        "token": jwt,
        "usuario": usuario
    })))
}
