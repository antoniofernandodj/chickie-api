use axum::{Extension, Json, extract::State, response::IntoResponse};
use std::sync::Arc;
use chickie_core::usecases::{CatalogoUsecase, CreateProdutoRequest};
use chickie_core::models::Usuario;
use crate::handlers::dto::AppError;
use crate::handlers::AppState;

pub async fn criar_produto(
    State(state): State<Arc<AppState>>,
    Extension(usuario_logado): Extension<Usuario>,
    Json(p): Json<CreateProdutoRequest>,
) -> Result<impl IntoResponse, AppError> {

    let loja_uuid = p.loja_uuid;
    let service = state.catalogo_service.clone();
    let usecase: CatalogoUsecase =
        CatalogoUsecase::new(service, loja_uuid, usuario_logado);

    let produto = usecase
        .criar_produto(p)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(produto))
}
