use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;
use rust_decimal::prelude::*;

use crate::api_handlers::{dto::AppError, AppState};
use chickie_core::{models::Usuario, usecases::AdminUsecase};

#[derive(Deserialize)]
pub struct AtualizarCupomRequest {
    pub codigo: String,
    pub descricao: String,
    pub tipo_desconto: String,
    pub valor_desconto: Option<f64>,
    pub valor_minimo: Option<f64>,
    pub data_validade: String,
    pub limite_uso: Option<i32>,
}

pub async fn atualizar_cupom(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<AtualizarCupomRequest>,
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

    uc.atualizar_cupom(
        uuid,
        p.codigo,
        p.descricao,
        p.tipo_desconto,
        p.valor_desconto.map(|v| Decimal::from_f64(v).unwrap_or(Decimal::ZERO)),
        p.valor_minimo.map(|v| Decimal::from_f64(v).unwrap_or(Decimal::ZERO)),
        p.data_validade,
        p.limite_uso,
    ).await?;
    Ok(StatusCode::NO_CONTENT)
}
