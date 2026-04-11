use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::api_handlers::{AppState, auth::AdminPermission, dto::AppError};

/// Lista todas as lojas criadas pelo admin autenticado
pub async fn listar_minhas_lojas(
    State(state): State<Arc<AppState>>,
    AdminPermission(usuario): AdminPermission,
) -> Result<impl IntoResponse, AppError> {

    let lojas = state
        .loja_service
        .listar_por_criador(usuario.uuid)
        .await?;

    Ok(Json(lojas))
}
