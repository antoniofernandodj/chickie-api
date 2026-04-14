use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};
use chickie_core::{
    models::Usuario,
    usecases::CatalogoUsecase,
    ports::to_proto::ToProto,
    proto,
};

pub async fn buscar_produto_por_uuid(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(usuario_logado): Extension<Usuario>,
) -> Result<Protobuf<proto::Produto>, AppError> {

    let usecase = CatalogoUsecase::new(
        state.catalogo_service.clone(),
        uuid,
        usuario_logado,
    );
    let produto = usecase.catalogo_service.buscar_produto_por_uuid(uuid).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    
    Ok(Protobuf(produto.to_proto()))
}
