use axum::{
    Extension, Json, extract::{Path, State}, response::IntoResponse
};
use uuid::Uuid;


use std::sync::Arc;
use crate::api_handlers::{AppState, dto::AppError};
use chickie_core::{models::Usuario};


pub async fn validar_cupom(
    Extension(_): Extension<Usuario>,
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
