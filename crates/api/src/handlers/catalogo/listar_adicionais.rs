use std::sync::Arc;

use axum::{Json, extract::{Path, State}, response::IntoResponse};
use uuid::Uuid;

use crate::handlers::{dto::AppError, AppState};

pub async fn listar_adicionais(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let adicionais = state.catalogo_service
        .listar_adicionais(loja_uuid)
        .await?;
    Ok(Json(adicionais))
}
