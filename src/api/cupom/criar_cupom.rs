use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse
};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;
use crate::{api::{AppState, dto::AppError}, models::Usuario};


#[derive(Deserialize)]
pub struct CriarCupomRequest {
    pub codigo: String,
    pub descricao: String,
    pub tipo_desconto: String,
    pub valor_desconto: Option<f64>,
    pub valor_minimo: Option<f64>,
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
