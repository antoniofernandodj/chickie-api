use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode
};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

use crate::{
    api::{dto::AppError, AppState},
    models::Usuario,
    usecases::MarketingUsecase
};

#[derive(Deserialize)]
pub struct AtualizarPromocaoRequest {
    pub nome: String,
    pub descricao: String,
    pub tipo_desconto: String,
    pub valor_desconto: Option<f64>,
    pub valor_minimo: Option<f64>,
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
