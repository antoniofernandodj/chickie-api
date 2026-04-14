use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    usecases::CatalogoUsecase,
    proto,
};

use crate::{
    handlers::{AppState, dto::AppError, protobuf::Protobuf},
};

pub async fn deletar_produto(
    State(state): State<Arc<AppState>>,
    Extension(usuario_logado): Extension<Usuario>,
    Path(uuid): Path<Uuid>,
) -> Result<Protobuf<proto::GenericResponse>, AppError> {

    let usecase = CatalogoUsecase::new(
        state.catalogo_service.clone(),
        uuid,
        usuario_logado,
    );
    usecase.deletar_produto(uuid).await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Protobuf(proto::GenericResponse {
        message: "Produto deletado com sucesso".to_string(),
        success: true,
    }))
}

/*

use axum::{Extension, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    usecases::CatalogoUsecase,
};

use crate::{
    handlers::{AppState, dto::AppError},
};

pub async fn deletar_produto(
    State(state): State<Arc<AppState>>,
    Extension(usuario_logado): Extension<Usuario>,
    Path(uuid): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = CatalogoUsecase::new(
        state.catalogo_service.clone(),
        uuid,
        usuario_logado,
    );
    usecase.deletar_produto(uuid).await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

*/
