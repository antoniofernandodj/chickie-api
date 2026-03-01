use axum::{Extension, Json, extract::{Path, State}};
use uuid::Uuid;
use std::sync::Arc;
use crate::{api::{CatalogoUsecase, dto::AppError}, models::{self, Usuario}};
use crate::api::AppState;


pub async fn listar_produtos(
    State(state): State<Arc<AppState>>,
    Extension(usuario_logado): Extension<Usuario>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<Json<Vec<models::Produto>>, AppError> {

    let service = Arc::new(state.catalogo_service.clone());
    let usecase: CatalogoUsecase =
        CatalogoUsecase::new(service, loja_uuid, usuario_logado);
    
    let produtos = usecase
        .listar_produtos()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(produtos))
}
