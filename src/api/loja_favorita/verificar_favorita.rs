use std::sync::Arc;

use axum::{Json, extract::{Path, State, Extension}, response::IntoResponse};
use uuid::Uuid;

use crate::{
    api::{dto::AppError, AppState},
    models::Usuario,
    usecases::VerificarLojaFavoritaUsecase
};

pub async fn verificar_favorita(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = VerificarLojaFavoritaUsecase::new(
        state.loja_favorita_service.clone(),
        usuario,
        loja_uuid
    );

    let favorita = usecase.executar().await?;

    Ok(Json(serde_json::json!({ "favorita": favorita })))
}
