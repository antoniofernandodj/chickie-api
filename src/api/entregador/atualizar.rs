use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

use crate::{api::{dto::AppError, AppState}, models::Usuario, usecases::AdminUsecase};

#[derive(Deserialize)]
pub struct AtualizarEntregadorRequest {
    pub usuario_uuid: Uuid,
    pub nome: Option<String>,
    pub celular: Option<String>,
    pub telefone: Option<String>,
    pub veiculo: Option<String>,
    pub placa: Option<String>,
}

pub async fn atualizar_entregador(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<AtualizarEntregadorRequest>,
) -> Result<impl IntoResponse, AppError> {
    let uc = AdminUsecase::new(
        state.ingrediente_service.clone(),
        state.horario_funcionamento_service.clone(),
        state.config_pedido_service.clone(),
        state.funcionario_service.clone(),
        state.entregador_service.clone(),
        state.marketing_service.clone(),
        usuario,
        loja_uuid,
    );
    uc.atualizar_entregador(uuid, p.usuario_uuid, p.nome, p.celular, p.telefone, p.veiculo, p.placa).await?;
    Ok(StatusCode::NO_CONTENT)
}
