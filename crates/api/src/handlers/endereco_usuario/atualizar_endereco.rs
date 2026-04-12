use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::models::Usuario;
use crate::handlers::{dto::AppError, AppState};

#[derive(Deserialize)]
pub struct UpdateEnderecoUsuarioRequest {
    pub cep: Option<String>,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
}

pub async fn atualizar_endereco(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<UpdateEnderecoUsuarioRequest>,
) -> Result<impl IntoResponse, AppError> {

    let endereco = state.endereco_usuario_service.atualizar_endereco(
        uuid,
        usuario.uuid,
        p.cep,
        p.logradouro,
        p.numero,
        p.complemento,
        p.bairro,
        p.cidade,
        p.estado,
        None,
        None
    ).await?;

    Ok(Json(endereco))
}
