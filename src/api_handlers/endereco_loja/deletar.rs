use axum::{Extension, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use uuid::Uuid;
use std::sync::Arc;

use crate::{api_handlers::{dto::AppError, AppState}, models::Usuario, usecases::AdminUsecase};

pub async fn deletar_endereco_loja(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, endereco_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
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
    uc.deletar_endereco(endereco_uuid).await?;
    Ok(StatusCode::NO_CONTENT)
}
