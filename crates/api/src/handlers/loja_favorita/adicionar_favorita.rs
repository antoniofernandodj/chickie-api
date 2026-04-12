use std::sync::Arc;

use axum::{Json, extract::{Path, State, Extension}, response::IntoResponse};
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    usecases::AdicionarLojaFavoritaUsecase
};
use crate::handlers::{dto::AppError, AppState};

pub async fn adicionar_favorita(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = AdicionarLojaFavoritaUsecase::new(
        state.loja_favorita_service.clone(),
        usuario,
        loja_uuid
    );

    let favorita = usecase.executar().await?;

    Ok(Json(favorita))
}
