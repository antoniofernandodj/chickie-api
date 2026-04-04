use std::sync::Arc;

use axum::{Json, extract::{State, Extension}, response::IntoResponse};

use crate::{
    api::{dto::AppError, AppState},
    models::Usuario,
    usecases::ListarLojasFavoritasUsecase
};

pub async fn listar_minhas_favoritas(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = ListarLojasFavoritasUsecase::new(
        Arc::new(state.loja_favorita_service.clone()),
        usuario
    );

    let favoritas = usecase.executar().await?;

    Ok(Json(favoritas))
}
