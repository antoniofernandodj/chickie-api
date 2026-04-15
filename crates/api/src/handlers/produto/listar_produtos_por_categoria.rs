use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};
use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;
use chickie_core::{
    models::Usuario,
    usecases::CatalogoUsecase,
    proto
};
use chickie_core::ports::to_proto::ToProto;

pub async fn listar_produtos_por_categoria(
    State(state): State<Arc<AppState>>,
    Path(categoria_uuid): Path<Uuid>,
    Extension(usuario_logado): Extension<Usuario>,
) -> Result<Protobuf<proto::ListarProdutosResponse>, AppError> {

    let usecase = CatalogoUsecase::new(
        state.catalogo_service.clone(),
        categoria_uuid,
        usuario_logado,
    );

    let produtos = usecase.catalogo_service.listar_produtos_por_categoria(categoria_uuid).await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let produtos_proto = produtos.iter().map(|p| p.to_proto()).collect();

    Ok(Protobuf(proto::ListarProdutosResponse { produtos: produtos_proto }))
}
