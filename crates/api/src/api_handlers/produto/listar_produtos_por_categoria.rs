use crate::api_handlers::{AppState, dto::AppError};
use axum::{Extension, Json, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;
use chickie_core::{
    models::{Usuario, Produto},
    usecases::CatalogoUsecase,
};

pub async fn listar_produtos_por_categoria(
    State(state): State<Arc<AppState>>,
    Path(categoria_uuid): Path<Uuid>,
    Extension(usuario_logado): Extension<Usuario>,
) -> Result<Json<Vec<Produto>>, AppError> {

    let usecase = CatalogoUsecase::new(
        state.catalogo_service.clone(),
        categoria_uuid,
        usuario_logado,
    );
    let produtos = usecase.catalogo_service.listar_produtos_por_categoria(categoria_uuid).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(produtos))
}
