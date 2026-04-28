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
    let celular_numerico: String = p.celular.chars().filter(|c| c.is_ascii_digit()).collect();
    let cpf_numerico: String = p.cpf.chars().filter(|c| c.is_ascii_digit()).collect();

    let usuario = state.usuario_service.registrar(
        p.nome,
        p.username,
        p.senha,
        p.email,
        celular_numerico,
        cpf_numerico,
        p.auth_method,
        p.classe
    ).await?;

    Ok(Json(usuario))
}