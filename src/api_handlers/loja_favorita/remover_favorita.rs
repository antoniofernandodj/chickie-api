use std::sync::Arc;

use axum::{Json, extract::{Path, State, Extension}, response::IntoResponse};
use uuid::Uuid;

use crate::{
    api_handlers::{dto::AppError, AppState},
    models::Usuario,
    usecases::RemoverLojaFavoritaUsecase
};

pub async fn remover_favorita(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = RemoverLojaFavoritaUsecase::new(
        state.loja_favorita_service.clone(),
        usuario,
        loja_uuid
    );

    usecase.executar().await?;

    Ok(Json(serde_json::json!({ "message": "Loja removida das favoritas" })))
}
