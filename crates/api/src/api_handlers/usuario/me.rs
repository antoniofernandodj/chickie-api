use axum::{Extension, Json, extract::State};
use serde_json::json;
use std::sync::Arc;

use crate::api_handlers::{AppState, dto::AppError};
use chickie_core::models::Usuario;
use chickie_core::repositories::Repository;

pub async fn me(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Buscar o usuário completo no banco para garantir dados atualizados
    let usuario_completo = state.usuario_repo
        .buscar_por_uuid(usuario.uuid)
        .await
        .map_err(|e| {
            tracing::error!("Erro ao buscar usuário completo: {}", e);
            AppError::Internal("Erro ao buscar dados do usuário".to_string())
        })?
        .ok_or_else(|| AppError::NotFound("Usuário não encontrado".to_string()))?;

    Ok(Json(json!({
        "uuid": usuario_completo.uuid,
        "nome": usuario_completo.nome,
        "username": usuario_completo.username,
        "email": usuario_completo.email,
        "celular": usuario_completo.celular,
        "classe": usuario_completo.classe,
        "ativo": usuario_completo.ativo,
        "passou_pelo_primeiro_acesso": usuario_completo.passou_pelo_primeiro_acesso,
        "criado_em": usuario_completo.criado_em,
        "atualizado_em": usuario_completo.atualizado_em,
        "modo_de_cadastro": usuario_completo.modo_de_cadastro
    })))
}
