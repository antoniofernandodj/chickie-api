use axum::{
    Extension, extract::{Path, State}, http::StatusCode
};
use std::sync::Arc;
use uuid::Uuid;

use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};
use chickie_core::models::Usuario;
use chickie_core::proto;

pub async fn atualizar_status_cupom(
    Extension(_): Extension<Usuario>,
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Protobuf(p): Protobuf<proto::AtualizarStatusCupomRequest>,
) -> Result<StatusCode, AppError> {

    let mut cupom = state.marketing_service.buscar_cupom_por_uuid(uuid).await?;

    if p.ativo {
        cupom.ativar();
    } else {
        cupom.desativar();
    }

    state.marketing_service.atualizar_cupom_status(cupom).await?;

    Ok(StatusCode::NO_CONTENT)
}

/*
use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::handlers::{AppState, dto::AppError};
use chickie_core::models::Usuario;


#[derive(Deserialize)]
pub struct AtualizarStatusCupomRequest {
    pub ativo: bool,
}


pub async fn atualizar_status_cupom(
    Extension(_): Extension<Usuario>,
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Json(p): Json<AtualizarStatusCupomRequest>,
) -> Result<impl IntoResponse, AppError> {

    let mut cupom = state.marketing_service.buscar_cupom_por_uuid(uuid).await?;

    if p.ativo {
        cupom.ativar();
    } else {
        cupom.desativar();
    }

    state.marketing_service.atualizar_cupom_status(cupom).await?;

    Ok(StatusCode::NO_CONTENT)
}
*/
