use std::sync::Arc;
use axum::{
    Json, extract::{Path, State, Extension}, response::IntoResponse
};
use uuid::Uuid;
use chickie_core::{
    models::Usuario,
    usecases::MarketingUsecase
};
use crate::handlers::{
    AppState, dto::{AppError, AtualizarAvaliacaoLojaRequest}
};

pub async fn atualizar_avaliacao_loja(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Json(payload): Json<AtualizarAvaliacaoLojaRequest>,
) -> Result<impl IntoResponse, AppError> {
    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        Uuid::nil(), // not needed for this operation
        usuario
    );
    let avaliacao = usecase.atualizar_avaliacao_loja(uuid, payload.nota, payload.comentario).await?;
    Ok(Json(avaliacao))
}
