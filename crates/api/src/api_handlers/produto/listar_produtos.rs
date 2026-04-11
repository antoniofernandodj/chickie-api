use crate::api_handlers::dto::AppError;
use crate::api_handlers::AppState;
use axum::{Extension, Json, extract::{Path, State}};
use uuid::Uuid;
use std::sync::Arc;
use chickie_core::{models::{self, Usuario}, usecases::CatalogoUsecase};


pub async fn listar_produtos(
    State(state): State<Arc<AppState>>,
    Extension(usuario_logado): Extension<Usuario>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<Json<Vec<models::Produto>>, AppError> {

    let service = state.catalogo_service.clone();
    let usecase: CatalogoUsecase =
        CatalogoUsecase::new(service, loja_uuid, usuario_logado);
    
    let produtos = usecase
        .listar_produtos()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(produtos))
}
