use axum::{
    extract::{State},
    response::{IntoResponse},
    Json
};
use std::sync::Arc;
use crate::{api::{AppState, CreateUsuarioRequest, dto::AppError}};


pub async fn criar_usuario(
    State(state): State<Arc<AppState>>,
    Json(p): Json<CreateUsuarioRequest>,
) -> Result<impl IntoResponse, AppError> {
    // O operador '?' converte o Err(String) do service em AppError::BadRequest automaticamente
    let usuario = state.usuario_service.registrar(
        p.nome,
        p.username,
        p.senha,
        p.email,
        p.telefone,
        p.auth_method
    ).await?;

    Ok(Json(usuario))
}