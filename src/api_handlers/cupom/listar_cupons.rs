use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use uuid::Uuid;
use std::sync::Arc;

use crate::{
    api_handlers::{dto::AppError, AppState},
    models::Usuario,
    usecases::MarketingUsecase
};

pub async fn listar_cupons(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    let cupons = usecase.listar_cupons().await?;

    Ok(Json(cupons))
}
