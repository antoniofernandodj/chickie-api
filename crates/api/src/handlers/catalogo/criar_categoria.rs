use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;
use chickie_core::{models::Usuario, proto};

pub async fn criar_categoria(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::CreateCategoriaRequest>,
) -> Result<Protobuf<proto::Categoria>, AppError> {

    let categoria = state.catalogo_service.criar_categoria(
        p.nome,
        if p.descricao.is_empty() { None } else { Some(p.descricao) },
        loja_uuid,
        if p.ordem == 0 { None } else { Some(p.ordem) },
        p.pizza_mode
    ).await?;

    Ok(Protobuf(categoria.to_proto()))
}
