use axum::{Extension, extract::State};
use std::sync::Arc;

use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;
use chickie_core::models::Usuario;
use chickie_core::repositories::Repository;
use chickie_core::proto;

pub async fn me(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::Usuario>, AppError> {
    // Buscar o usuário completo no banco para garantir dados atualizados
    let mut usuario_completo = state.usuario_repo
        .buscar_por_uuid(usuario.uuid)
        .await
        .map_err(|e| {
            tracing::error!("Erro ao buscar usuário completo: {}", e);
            AppError::Internal("Erro ao buscar dados do usuário".to_string())
        })?
        .ok_or_else(|| AppError::NotFound("Usuário não encontrado".to_string()))?;

    // Se o email corresponde ao OWNER_EMAIL, sobrescrever a classe para "owner"
    let owner_email = std::env::var("OWNER_EMAIL").unwrap_or_default();
    if !owner_email.is_empty() && usuario_completo.email == owner_email {
        usuario_completo.classe = "owner".to_string();
    }

    Ok(Protobuf(usuario_completo.to_proto()))
}
