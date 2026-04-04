use axum::{
    extract::{State, Extension},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::{
    api::{dto::AppError, AppState},
    models::Usuario,
};

/// Lista todas as lojas criadas pelo admin autenticado
pub async fn listar_minhas_lojas(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    // Apenas administradores podem listar suas lojas
    if !usuario.is_administrador() {
        return Err(AppError::Unauthorized(
            "Apenas administradores podem listar lojas".to_string()
        ));
    }

    let lojas = state
        .loja_service
        .listar_por_criador(usuario.uuid)
        .await?;

    Ok(Json(lojas))
}
