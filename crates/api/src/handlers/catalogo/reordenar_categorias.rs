use axum::{Extension, Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use std::{collections::HashSet, sync::Arc};
use uuid::Uuid;

use chickie_core::models::Usuario;
use crate::handlers::{dto::AppError, AppState};

#[derive(Deserialize)]
pub struct ReordenarItem {
    pub categoria_uuid: Uuid,
    pub ordem: i32,
}

pub async fn reordenar_categorias(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
    Json(p): Json<Vec<ReordenarItem>>,
) -> Result<impl IntoResponse, AppError> {

    if p.is_empty() {
        return Err(
            AppError::BadRequest(
                "Lista de reordenação não pode ser vazia".to_string()
            )
        );
    }

    let mut ordens_vistas: HashSet<i32> = HashSet::new();
    for item in &p {
        if item.ordem < 1 {
            return Err(
                AppError::BadRequest(
                    format!("Ordem deve ser maior que zero, recebido: {}", item.ordem)
                )
            );
        }
        if !ordens_vistas.insert(item.ordem) {
            return Err(
                AppError::BadRequest(
                    format!("Ordem {} duplicada na requisição", item.ordem)
                )
            );
        }
    }

    let reordenacoes = p.into_iter().map(|i| (i.categoria_uuid, i.ordem)).collect();

    state.catalogo_service.reordenar_categorias(Some(loja_uuid), reordenacoes).await?;

    Ok(StatusCode::NO_CONTENT)
}
