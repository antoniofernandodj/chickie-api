use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse
};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;
use rust_decimal::Decimal;

use crate::{
    api_handlers::{dto::AppError, AppState},
    models::Usuario,
    usecases::MarketingUsecase
};

#[derive(Deserialize)]
pub struct CriarPromocaoRequest {
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

pub async fn criar_promocao(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<CriarPromocaoRequest>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    let promocao = usecase.criar_promocao(
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
        p.prioridade
    ).await?;

    Ok(Json(promocao))
}
