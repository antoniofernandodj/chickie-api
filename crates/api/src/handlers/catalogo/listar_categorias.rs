use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{models::Usuario, ports::to_proto::ToProto, proto};
use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};

pub async fn listar_categorias(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
) -> Result<Protobuf<proto::ListarCategoriasResponse>, AppError> {

    let categorias = state.catalogo_service.listar_categorias(loja_uuid).await?;
    Ok(Protobuf(proto::ListarCategoriasResponse {
        categorias: categorias.into_iter().map(|c| c.to_proto()).collect(),
    }))
}
