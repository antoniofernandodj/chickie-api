use crate::handlers::dto::AppError;
use crate::handlers::{AppState, protobuf::Protobuf};
use axum::{Extension, extract::{Path, State}};
use chickie_core::ports::to_proto::to_proto_vec;
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

    let response
        = Protobuf(proto::ListarProdutosResponse { produtos: to_proto_vec(produtos) });

    // Usa a função genérica para converter
    Ok(response)
}
