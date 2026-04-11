use axum::{
    Extension, extract::{Path, State}, response::IntoResponse, http::StatusCode
};
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::Usuario,
    usecases::MarketingUsecase
};
use crate::api_handlers::{dto::AppError, AppState};

pub async fn deletar_promocao(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    usecase.deletar_promocao(uuid).await?;

    Ok(StatusCode::NO_CONTENT)
}
