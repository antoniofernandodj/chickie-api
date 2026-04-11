use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    usecases::ListarEnderecosEntregaPorLojaUsecase
};
use crate::api_handlers::{dto::AppError, AppState};

pub async fn listar_por_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = ListarEnderecosEntregaPorLojaUsecase::new(
        state.endereco_entrega_service.clone(),
        usuario,
        loja_uuid
    );

    let enderecos = usecase.executar().await?;

    Ok(Json(enderecos))
}
