use axum::extract::Path;
use axum::{Extension, Json, extract::State, response::IntoResponse};
use uuid::Uuid;
use std::sync::Arc;
use chickie_core::usecases::{AtualizarProdutoRequest, CatalogoUsecase};
use chickie_core::models::Usuario;
use crate::handlers::dto::AppError;
use crate::handlers::AppState;

pub async fn atualizar_produto(
    State(state): State<Arc<AppState>>,
    Extension(usuario_logado): Extension<Usuario>,
    Path(uuid): Path<Uuid>,
    Json(p): Json<AtualizarProdutoRequest>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = CatalogoUsecase::new(
        state.catalogo_service.clone(),
        uuid,
        usuario_logado,
    );

    let produto = usecase
        .atualizar_produto(uuid, p)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(produto))
}
