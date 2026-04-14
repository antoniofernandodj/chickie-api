use axum::{Extension, Json, extract::State};
use std::sync::Arc;
use chickie_core::usecases::{CatalogoUsecase, CreateProdutoRequest};
use chickie_core::models::Usuario;
use chickie_core::proto;
use crate::handlers::dto::AppError;
use crate::handlers::{AppState, protobuf::Protobuf};

pub async fn criar_produto(
    State(state): State<Arc<AppState>>,
    Extension(usuario_logado): Extension<Usuario>,
    Json(p): Json<CreateProdutoRequest>,
) -> Result<Protobuf<proto::Produto>, AppError> {

    let loja_uuid = p.loja_uuid;
    let service = state.catalogo_service.clone();
    let usecase: CatalogoUsecase =
        CatalogoUsecase::new(service, loja_uuid, usuario_logado);

    let produto = usecase
        .criar_produto(p)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Protobuf(produto.to_proto()))
}
