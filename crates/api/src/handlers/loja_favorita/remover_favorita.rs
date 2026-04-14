use std::sync::Arc;

use axum::extract::{Path, State, Extension};
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    usecases::RemoverLojaFavoritaUsecase,
    proto,
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn remover_favorita(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::GenericResponse>, AppError> {

    let usecase = RemoverLojaFavoritaUsecase::new(
        state.loja_favorita_service.clone(),
        usuario,
        loja_uuid
    );

    usecase.executar().await?;

    Ok(Protobuf(proto::GenericResponse {
        message: "Loja removida das favoritas".to_string(),
        success: true,
    }))
}
