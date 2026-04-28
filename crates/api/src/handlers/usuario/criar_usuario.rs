use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use serde_json::json;
use crate::handlers::{AppState, CreateUsuarioRequest, dto::AppError};

pub async fn criar_usuario(
    State(state): State<Arc<AppState>>,
    Json(p): Json<CreateUsuarioRequest>,
) -> Result<impl IntoResponse, AppError> {
    let celular_numerico: String = p.celular.chars().filter(|c| c.is_ascii_digit()).collect();
    let cpf_numerico: String = p.cpf.chars().filter(|c| c.is_ascii_digit()).collect();

    state.usuario_service.iniciar_cadastro(
        p.nome,
        p.username,
        p.senha,
        p.email.clone(),
        celular_numerico,
        cpf_numerico,
        p.auth_method,
        p.classe,
    ).await?;

    Ok((
        StatusCode::ACCEPTED,
        Json(json!({
            "message": format!(
                "Email de verificação enviado para {}. Você tem 1 hora para confirmar.",
                p.email
            )
        })),
    ))
}
