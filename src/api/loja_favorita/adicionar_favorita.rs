use std::sync::Arc;

use axum::{Json, extract::{Path, State, Extension}, response::IntoResponse};
use uuid::Uuid;

use crate::{
    api::{dto::AppError, AppState},
    models::Usuario,
    usecases::AdicionarLojaFavoritaUsecase
};

pub async fn adicionar_favorita(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = AdicionarLojaFavoritaUsecase::new(
        Arc::new(state.loja_favorita_service.clone()),
        usuario,
        loja_uuid
    );

    let favorita = usecase.executar().await?;

    Ok(Json(favorita))
}
