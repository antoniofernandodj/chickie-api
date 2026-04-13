use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::handlers::{auth::AdminPermission, dto::AppError, AppState};

// ============================================================================
// Marcar loja para remoção (soft delete)
// ============================================================================

pub async fn marcar_loja_remocao(
    State(state): State<Arc<AppState>>,
    AdminPermission(_admin): AdminPermission,
    Path(loja_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.loja_service
        .marcar_para_remocao(loja_uuid)
        .await
        .map_err(|e| AppError::BadRequest(e))?;

    tracing::info!("Loja {} marcada para remoção", loja_uuid);

    Ok((StatusCode::NO_CONTENT, ()))
}

// ============================================================================
// Desmarcar remoção de loja
// ============================================================================

pub async fn desmarcar_loja_remocao(
    State(state): State<Arc<AppState>>,
    AdminPermission(_admin): AdminPermission,
    Path(loja_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.loja_service
        .desmarcar_remocao(loja_uuid)
        .await
        .map_err(|e| AppError::BadRequest(e))?;

    tracing::info!("Remoção pendente da loja {} desmarcada", loja_uuid);

    Ok((StatusCode::NO_CONTENT, ()))
}

// ============================================================================
// Alternar status ativo da loja (bloquear/desbloquear admin)
// ============================================================================

#[derive(Deserialize, Serialize)]
pub struct AlternarAtivoRequest {
    pub ativo: bool,
}

pub async fn alternar_loja_ativo(
    State(state): State<Arc<AppState>>,
    AdminPermission(_admin): AdminPermission,
    Path(loja_uuid): Path<Uuid>,
    Json(body): Json<AlternarAtivoRequest>,
) -> Result<impl IntoResponse, AppError> {
    state.loja_service
        .alternar_ativo(loja_uuid, body.ativo)
        .await
        .map_err(|e| AppError::BadRequest(e))?;

    let acao = if body.ativo { "ativada" } else { "desativada" };
    tracing::info!("Loja {} {} ({})", loja_uuid, acao, body.ativo);

    Ok(Json(serde_json::json!({
        "message": format!("Loja {} com sucesso", acao),
        "ativo": body.ativo
    })))
}

// ============================================================================
// Toggle bloqueado da loja (bloquear/desbloquear operação)
// ============================================================================

pub async fn toggle_loja_bloqueado(
    State(state): State<Arc<AppState>>,
    AdminPermission(_admin): AdminPermission,
    Path(loja_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let bloqueado = state.loja_service
        .toggle_bloqueado(loja_uuid)
        .await
        .map_err(|e| AppError::BadRequest(e))?;

    let acao = if bloqueado { "bloqueada" } else { "desbloqueada" };
    tracing::info!("Loja {} {} (bloqueado={})", loja_uuid, acao, bloqueado);

    Ok(Json(serde_json::json!({
        "message": format!("Loja {} com sucesso", acao),
        "bloqueado": bloqueado
    })))
}
