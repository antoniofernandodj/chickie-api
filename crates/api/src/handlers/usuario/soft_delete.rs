use axum::{
    extract::{Path, State},
    Extension,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::handlers::{auth::OwnerPermission, dto::AppError, AppState};
use chickie_core::models::Usuario;

// ============================================================================
// Helper: verifica se o usuário logado é o próprio usuário OU o owner
// ============================================================================

fn is_self_or_owner(logged_in: &Usuario, target_uuid: Uuid) -> Result<(), AppError> {
    let owner_email = std::env::var("OWNER_EMAIL").unwrap_or_default();
    let is_self = logged_in.uuid == target_uuid;
    let is_owner_by_class = logged_in.is_owner();
    let is_owner_by_email = !owner_email.is_empty() && logged_in.email == owner_email;

    if is_self || is_owner_by_class || is_owner_by_email {
        Ok(())
    } else {
        Err(AppError::Unauthorized(
            "Apenas o próprio usuário ou o dono da plataforma podem realizar essa ação.".to_string(),
        ))
    }
}

// ============================================================================
// Marcar usuário para remoção (soft delete)
// ============================================================================

pub async fn marcar_usuario_remocao(
    State(state): State<Arc<AppState>>,
    Extension(logged_in): Extension<Usuario>,
    Path(usuario_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    is_self_or_owner(&logged_in, usuario_uuid)?;

    state.usuario_service
        .marcar_para_remocao(usuario_uuid)
        .await
        .map_err(|e| AppError::BadRequest(e))?;

    let acao = if logged_in.uuid == usuario_uuid { "auto" } else { "por owner" };
    tracing::info!("Usuário {} marcado para remoção ({})", usuario_uuid, acao);

    Ok((StatusCode::NO_CONTENT, ()))
}

// ============================================================================
// Desmarcar remoção de usuário
// ============================================================================

pub async fn desmarcar_usuario_remocao(
    State(state): State<Arc<AppState>>,
    Extension(logged_in): Extension<Usuario>,
    Path(usuario_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    is_self_or_owner(&logged_in, usuario_uuid)?;

    state.usuario_service
        .desmarcar_remocao(usuario_uuid)
        .await
        .map_err(|e| AppError::BadRequest(e))?;

    let acao = if logged_in.uuid == usuario_uuid { "auto" } else { "por owner" };
    tracing::info!("Remoção pendente do usuário {} desmarcada ({})", usuario_uuid, acao);

    Ok((StatusCode::NO_CONTENT, ()))
}

// ============================================================================
// Alternar status ativo do usuário (bloquear/desbloquear)
// ============================================================================

#[derive(Deserialize, Serialize)]
pub struct AlternarAtivoRequest {
    pub ativo: bool,
}

pub async fn alternar_usuario_ativo(
    State(state): State<Arc<AppState>>,
    OwnerPermission(_owner): OwnerPermission,
    Path(usuario_uuid): Path<Uuid>,
    Json(body): Json<AlternarAtivoRequest>,
) -> Result<impl IntoResponse, AppError> {
    state.usuario_service
        .alternar_ativo(usuario_uuid, body.ativo)
        .await
        .map_err(|e| AppError::BadRequest(e))?;

    let acao = if body.ativo { "ativado" } else { "desativado" };
    tracing::info!("Usuário {} {} ({})", usuario_uuid, acao, body.ativo);

    Ok(Json(serde_json::json!({
        "message": format!("Usuário {} com sucesso", acao),
        "ativo": body.ativo
    })))
}

// ============================================================================
// Toggle bloqueado do usuário (bloquear/desbloquear login)
// ============================================================================

pub async fn toggle_usuario_bloqueado(
    State(state): State<Arc<AppState>>,
    OwnerPermission(_owner): OwnerPermission,
    Path(usuario_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let bloqueado = state.usuario_service
        .toggle_bloqueado(usuario_uuid)
        .await
        .map_err(|e| AppError::BadRequest(e))?;

    let acao = if bloqueado { "bloqueado" } else { "desbloqueado" };
    tracing::info!("Usuário {} {} (bloqueado={})", usuario_uuid, acao, bloqueado);

    Ok(Json(serde_json::json!({
        "message": format!("Usuário {} com sucesso", acao),
        "bloqueado": bloqueado
    })))
}
