use std::sync::Arc;

use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use uuid::Uuid;

use crate::{
    api::{dto::AppError, AppState},
    models::Usuario,
    usecases::CatalogoUsecase
};

pub async fn listar_adicionais_disponiveis(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = CatalogoUsecase::new(
        state.catalogo_service.clone(),
        loja_uuid,
        usuario
    );

    let adicionais = usecase.listar_adicionais_disponiveis().await?;

    Ok(Json(adicionais))
}
