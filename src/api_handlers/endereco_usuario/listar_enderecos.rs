use axum::{Extension, Json, extract::State, response::IntoResponse};
use std::sync::Arc;

use crate::{
    api_handlers::{dto::AppError, AppState},
    models::Usuario
};

pub async fn listar_enderecos(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let enderecos = state.endereco_usuario_service
        .listar_enderecos(usuario.uuid)
        .await?;

    Ok(Json(enderecos))
}
