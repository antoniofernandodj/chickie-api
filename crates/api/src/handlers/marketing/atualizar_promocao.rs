use axum::{
    Extension, extract::{Path, State}, http::StatusCode
};
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::Usuario,
    usecases::MarketingUsecase,
    proto
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn atualizar_promocao(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::AtualizarPromocaoRequest>,
) -> Result<StatusCode, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    let produto_uuid = if p.produto_uuid.is_empty() { None } else { Some(Uuid::parse_str(&p.produto_uuid).unwrap_or_default()) };
    let categoria_uuid = if p.categoria_uuid.is_empty() { None } else { Some(Uuid::parse_str(&p.categoria_uuid).unwrap_or_default()) };
    let valor_desconto = if p.valor_desconto.is_empty() { None } else { Some(p.valor_desconto.parse().unwrap_or_default()) };
    let valor_minimo = if p.valor_minimo.is_empty() { None } else { Some(p.valor_minimo.parse().unwrap_or_default()) };
    let dias_semana: Option<Vec<u8>> = if p.dias_semana_validos.is_empty() { None } else { Some(p.dias_semana_validos.iter().map(|&x| x as u8).collect()) };

    usecase.atualizar_promocao(
        uuid,
        p.nome,
        p.descricao,
        p.tipo_desconto,
        valor_desconto,
        valor_minimo,
        p.data_inicio,
        p.data_fim,
        dias_semana,
        p.tipo_escopo,
        produto_uuid,
        categoria_uuid,
        p.prioridade,
    ).await?;

    Ok(StatusCode::NO_CONTENT)
}

/*
use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode
};
use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::Usuario,
    usecases::MarketingUsecase
};
use crate::handlers::{dto::AppError, AppState};

#[derive(Deserialize)]
pub struct AtualizarPromocaoRequest {
    pub nome: String,
    pub descricao: String,
    pub tipo_desconto: String,
    pub valor_desconto: Option<Decimal>,
    pub valor_minimo: Option<Decimal>,
    pub data_inicio: String,
    pub data_fim: String,
    pub dias_semana_validos: Option<Vec<u8>>,
    pub tipo_escopo: String,
    pub produto_uuid: Option<Uuid>,
    pub categoria_uuid: Option<Uuid>,
    pub prioridade: i32,
}

pub async fn atualizar_promocao(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<AtualizarPromocaoRequest>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    usecase.atualizar_promocao(
        uuid,
        p.nome,
        p.descricao,
        p.tipo_desconto,
        p.valor_desconto,
        p.valor_minimo,
        p.data_inicio,
        p.data_fim,
        p.dias_semana_validos,
        p.tipo_escopo,
        p.produto_uuid,
        p.categoria_uuid,
        p.prioridade,
    ).await?;

    Ok(StatusCode::NO_CONTENT)
}
*/
