use crate::handlers::dto::AppError;
use crate::handlers::{AppState, protobuf::Protobuf};
use axum::{Extension, extract::{Path, State}};
use uuid::Uuid;
use std::sync::Arc;
use chickie_core::{models::Usuario, usecases::CatalogoUsecase, proto};


pub async fn listar_produtos(
    State(state): State<Arc<AppState>>,
    Extension(usuario_logado): Extension<Usuario>,
    Path(loja_uuid): Path<Uuid>,
) -> Result<Protobuf<proto::ListarProdutosResponse>, AppError> {

    let service = state.catalogo_service.clone();
    let usecase: CatalogoUsecase =
        CatalogoUsecase::new(service, loja_uuid, usuario_logado);
    
    let produtos = usecase
        .listar_produtos()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Protobuf(proto::ListarProdutosResponse {
        produtos: produtos.iter().map(|p| p.to_proto()).collect(),
    }))
}
