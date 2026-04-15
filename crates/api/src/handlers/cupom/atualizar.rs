use axum::{Extension, extract::{Path, State}, http::StatusCode};
use uuid::Uuid;
use std::sync::Arc;
use rust_decimal::prelude::*;

use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::{models::Usuario, usecases::AdminUsecase, proto};

pub async fn atualizar_cupom(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::AtualizarCupomRequest>,
) -> Result<StatusCode, AppError> {
    let loja_uuid = Uuid::parse_str(&p.loja_uuid)
        .map_err(|e| AppError::BadRequest(format!("Invalid loja_uuid: {}", e)))?;

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

    let valor_desconto = if p.valor_desconto.is_empty() { None } else { Some(p.valor_desconto.parse::<Decimal>().unwrap_or(Decimal::ZERO)) };
    let valor_minimo = if p.valor_minimo.is_empty() { None } else { Some(p.valor_minimo.parse::<Decimal>().unwrap_or(Decimal::ZERO)) };
    let limite_uso = if p.limite_uso == 0 { None } else { Some(p.limite_uso) };

    uc.atualizar_cupom(
        uuid,
        p.codigo,
        p.descricao,
        p.tipo_desconto,
        valor_desconto,
        valor_minimo,
        p.data_validade,
        limite_uso,
    ).await?;
    Ok(StatusCode::NO_CONTENT)
}

/*
use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;
use rust_decimal::prelude::*;

use crate::handlers::{dto::AppError, AppState};
use chickie_core::{models::Usuario, usecases::AdminUsecase};

#[derive(Deserialize)]
pub struct AtualizarCupomRequest {
    pub loja_uuid: Uuid,
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
    Path(uuid): Path<Uuid>,
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
        p.loja_uuid,
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
*/
