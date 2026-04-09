use std::sync::Arc;

use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::{dto::AppError, AppState},
    models::Usuario,
};

#[derive(Deserialize)]
pub struct AtualizarDisponibilidadeRequest {
    pub disponivel: bool,
}

pub async fn atualizar_disponibilidade(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, adicional_uuid)): Path<(Uuid, Uuid)>,
    Extension(_): Extension<Usuario>,
    Json(p): Json<AtualizarDisponibilidadeRequest>,
) -> Result<impl IntoResponse, AppError> {

    state.catalogo_service.atualizar_disponibilidade(adicional_uuid, loja_uuid, p.disponivel).await?;

    Ok(StatusCode::NO_CONTENT)
}
