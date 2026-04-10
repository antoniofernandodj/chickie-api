use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

use crate::{api::{dto::AppError, AppState}, models::Usuario, usecases::AdminUsecase};

#[derive(Deserialize)]
pub struct TrocarEmailSenhaRequest {
    pub novo_email: Option<String>,
    pub nova_senha: Option<String>,
}

pub async fn funcionario_trocar_email_senha(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, usuario_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<TrocarEmailSenhaRequest>,
) -> Result<impl IntoResponse, AppError> {
    let uc = AdminUsecase::new(
        state.ingrediente_service.clone(),
        state.horario_funcionamento_service.clone(),
        state.config_pedido_service.clone(),
        state.funcionario_service.clone(),
        state.entregador_service.clone(),
        state.marketing_service.clone(),
        state.endereco_loja_service.clone(),
        usuario,
        loja_uuid,
    );
    uc.funcionario_trocar_email_senha(usuario_uuid, p.novo_email, p.nova_senha).await?;
    Ok(StatusCode::NO_CONTENT)
}
