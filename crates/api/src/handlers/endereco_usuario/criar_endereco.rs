use axum::{Extension, Json, extract::State, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;

use chickie_core::models::Usuario;
use crate::handlers::{dto::AppError, AppState};

#[derive(Deserialize)]
pub struct CreateEnderecoUsuarioRequest {
    pub cep: Option<String>,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
}

pub async fn criar_endereco(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<CreateEnderecoUsuarioRequest>,
) -> Result<impl IntoResponse, AppError> {

    let endereco = state.endereco_usuario_service.criar_endereco(
        usuario.uuid,
        p.cep,
        p.logradouro,
        p.numero,
        p.complemento,
        p.bairro,
        p.cidade,
        p.estado
    ).await?;

    Ok(Json(endereco))
}
