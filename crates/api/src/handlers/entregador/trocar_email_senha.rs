use axum::{Extension, extract::{Path, State}, http::StatusCode};
use std::sync::Arc;
use uuid::Uuid;

use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::{models::Usuario, usecases::AdminUsecase, proto};

pub async fn entregador_trocar_email_senha(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, usuario_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::TrocarEmailSenhaRequest>,
) -> Result<StatusCode, AppError> {
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
    let novo_email = if p.novo_email.is_empty() { None } else { Some(p.novo_email.clone()) };
    let nova_senha = if p.nova_senha.is_empty() { None } else { Some(p.nova_senha.clone()) };
    uc.entregador_trocar_email_senha(usuario_uuid, novo_email, nova_senha).await?;
    Ok(StatusCode::NO_CONTENT)
}

/*
use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState};
use chickie_core::{models::Usuario, usecases::AdminUsecase};

#[derive(Deserialize)]
pub struct TrocarEmailSenhaRequest {
    pub novo_email: Option<String>,
    pub nova_senha: Option<String>,
}

pub async fn entregador_trocar_email_senha(
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
    uc.entregador_trocar_email_senha(usuario_uuid, p.novo_email, p.nova_senha).await?;
    Ok(StatusCode::NO_CONTENT)
}
*/
