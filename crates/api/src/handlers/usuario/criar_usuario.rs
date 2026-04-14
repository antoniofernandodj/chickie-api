use axum::{
    extract::{State},
};
use std::sync::Arc;
use chickie_core::ports::to_proto::ToProto;
use chickie_core::proto;
use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};


pub async fn criar_usuario(
    State(state): State<Arc<AppState>>,
    Protobuf(p): Protobuf<proto::CreateUsuarioRequest>,
) -> Result<Protobuf<proto::Usuario>, AppError> {
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
        if p.classe.is_empty() { None } else { Some(p.classe) }
    ).await?;

    Ok(Protobuf(usuario.to_proto()))
}
