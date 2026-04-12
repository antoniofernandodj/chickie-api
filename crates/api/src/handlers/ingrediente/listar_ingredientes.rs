use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use uuid::Uuid;
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState};
use chickie_core::{models::Usuario, usecases::AdminUsecase};

pub async fn listar_ingredientes(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {
    let uc = AdminUsecase::new(state.ingrediente_service.clone(), state.horario_funcionamento_service.clone(), state.config_pedido_service.clone(), state.funcionario_service.clone(), state.entregador_service.clone(), state.marketing_service.clone(), state.endereco_loja_service.clone(), usuario, loja_uuid);
    Ok(Json(uc.listar_ingredientes().await?))
}
