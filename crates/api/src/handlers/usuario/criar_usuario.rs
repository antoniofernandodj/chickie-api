use axum::{
    extract::{State},
    response::{IntoResponse},
    Json
};
use std::sync::Arc;
use crate::handlers::{AppState, CreateUsuarioRequest, dto::AppError};


pub async fn criar_usuario(
    State(state): State<Arc<AppState>>,
    Json(p): Json<CreateUsuarioRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Filtrar celular: manter apenas dígitos numéricos
    let celular_numerico: String = p.celular.chars().filter(|c| c.is_ascii_digit()).collect();

    // O operador '?' converte o Err(String) do service em AppError::BadRequest automaticamente
    let usuario = state.usuario_service.registrar(
        p.nome,
        p.username,
        p.senha,
        p.email,
        celular_numerico,
        p.auth_method,
        p.classe
    ).await?;

    Ok(Json(usuario))
}