use std::sync::Arc;

use axum::{Extension, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use uuid::Uuid;

use crate::{
    api::{dto::AppError, AppState},
    models::Usuario,
    usecases::CatalogoUsecase
};

pub async fn marcar_indisponivel(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, adicional_uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = CatalogoUsecase::new(
        Arc::new(state.catalogo_service.clone()),
        loja_uuid,
        usuario
    );

    usecase.marcar_adicional_indisponivel(adicional_uuid).await?;

    Ok(StatusCode::NO_CONTENT)
}
