use axum::{
    Extension, Json, extract::State, response::IntoResponse
};
use serde::Deserialize;
use std::sync::Arc;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::handlers::{AppState, dto::AppError};
use chickie_core::models::Usuario;


#[derive(Deserialize)]
pub struct CriarCupomGenericoRequest {
    pub loja_uuid: Uuid,
    pub codigo: String,
    pub descricao: String,
    pub tipo_desconto: String,
    pub valor_desconto: Option<Decimal>,
    pub valor_minimo: Option<Decimal>,
    pub data_validade: String,
    pub limite_uso: Option<i32>,
}


pub async fn criar_cupom_generico(
    Extension(_): Extension<Usuario>,
    State(state): State<Arc<AppState>>,
    Json(p): Json<CriarCupomGenericoRequest>,
) -> Result<impl IntoResponse, AppError> {

    let cupom = state.marketing_service.criar_cupom(
        p.loja_uuid,
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
