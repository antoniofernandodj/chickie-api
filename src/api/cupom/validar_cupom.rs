use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse}
};
use uuid::Uuid;


use std::sync::Arc;
use crate::{api::{AppState, dto::AppError}};


pub async fn validar_cupom(
    State(state): State<Arc<AppState>>,
    Path(codigo): Path<String>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {

    let cupom = state.cupom_repo
        .buscar_por_codigo(&codigo, loja_uuid)
        .await
        // Erro de banco -> Internal
        .map_err(|e| AppError::Internal(e.to_string()))?
        // None -> NotFound
        .ok_or_else(|| AppError::NotFound("Cupom não encontrado".into()))?;

    Ok(Json(cupom))
}
