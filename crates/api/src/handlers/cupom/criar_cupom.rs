use axum::{
    Extension, extract::{Path, State}
};
use uuid::Uuid;
use std::sync::Arc;
use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};
use chickie_core::{models::Usuario, proto};
use chickie_core::ports::to_proto::ToProto;
use rust_decimal::Decimal;

pub async fn criar_cupom(
    Extension(_): Extension<Usuario>,
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Protobuf(p): Protobuf<proto::CriarCupomRequest>,
) -> Result<Protobuf<proto::Cupom>, AppError> {

    let valor_desconto = if p.valor_desconto.is_empty() { None } else { Some(p.valor_desconto.parse::<Decimal>().unwrap_or_default()) };
    let valor_minimo = if p.valor_minimo.is_empty() { None } else { Some(p.valor_minimo.parse::<Decimal>().unwrap_or_default()) };
    let limite_uso = if p.limite_uso == 0 { None } else { Some(p.limite_uso) };

    let cupom = state.marketing_service.criar_cupom(
        loja_uuid,
        p.codigo,
        p.descricao,
        p.tipo_desconto,
        valor_desconto,
        valor_minimo,
        p.data_validade,
        limite_uso
    ).await?;

    Ok(Protobuf(cupom.to_proto()))
}

/*
use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse
};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;
use rust_decimal::Decimal;
use crate::handlers::{AppState, dto::AppError};
use chickie_core::{models::Usuario};


#[derive(Deserialize)]
pub struct CriarCupomRequest {
    pub codigo: String,
    pub descricao: String,
    pub tipo_desconto: String,
    pub valor_desconto: Option<Decimal>,
    pub valor_minimo: Option<Decimal>,
    pub data_validade: String,
    pub limite_uso: Option<i32>,
}


pub async fn criar_cupom(
    Extension(_): Extension<Usuario>,
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Json(p): Json<CriarCupomRequest>,
) -> Result<impl IntoResponse, AppError> {

    let cupom = state.marketing_service.criar_cupom(
        loja_uuid,
        p.codigo,
        p.descricao,
        p.tipo_desconto,
        p.valor_desconto,
        p.valor_minimo,
        p.data_validade,
        p.limite_uso
    ).await?;

    Ok(Json(cupom))
}
*/
