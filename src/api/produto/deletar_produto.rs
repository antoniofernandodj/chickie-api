use axum::{Extension, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::{AppState, dto::AppError},
    models::Usuario,
    usecases::CatalogoUsecase,
};

pub async fn deletar_produto(
    State(state): State<Arc<AppState>>,
    Extension(usuario_logado): Extension<Usuario>,
    Path(uuid): Path<Uuid>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = CatalogoUsecase::new(state.catalogo_service.clone(), loja_uuid, usuario_logado);
    usecase.deletar_produto(uuid).await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
