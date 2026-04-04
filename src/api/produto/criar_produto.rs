use axum::extract::Path;
use axum::{Extension, Json, extract::State, response::IntoResponse};
use uuid::Uuid;
use std::sync::Arc;
use crate::usecases::{CatalogoUsecase, CreateProdutoRequest};
use crate::models::Usuario;
use crate::api::dto::AppError;
use crate::api::AppState;




pub async fn criar_produto(
    State(state): State<Arc<AppState>>,
    Extension(usuario_logado): Extension<Usuario>,
    Path(loja_uuid): Path<Uuid>,
    Json(p): Json<CreateProdutoRequest>,
) -> Result<impl IntoResponse, AppError> {

    let service = state.catalogo_service.clone();
    let usecase: CatalogoUsecase =
        CatalogoUsecase::new(service, loja_uuid, usuario_logado);

    let produto = usecase
        .criar_produto(p)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(produto))
}
