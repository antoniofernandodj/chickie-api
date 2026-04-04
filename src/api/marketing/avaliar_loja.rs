use std::sync::Arc;

use axum::{
    Json, extract::{Path, State, Extension}, response::IntoResponse
};
use uuid::Uuid;

use crate::{
    api::{
        dto::{AppError, AvaliarLojaRequest},
        AppState
    },
    models::Usuario,
    usecases::MarketingUsecase
};

pub async fn avaliar_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Json(payload): Json<AvaliarLojaRequest>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    let avaliacao = usecase.avaliar_loja(
        payload.nota,
        payload.comentario
    ).await?;

    Ok(Json(avaliacao))
}
