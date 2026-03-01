use axum::{
    Json, extract::{Path, State}, response::IntoResponse
};
use uuid::Uuid;


use std::sync::Arc;
use crate::{api::dto::AppError, models::Cupom};
use crate::api::AppState;



pub async fn criar_cupom(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Json(p): Json<Cupom>,
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