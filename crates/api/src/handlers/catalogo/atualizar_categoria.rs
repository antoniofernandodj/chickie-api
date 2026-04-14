use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;
use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;
use chickie_core::{models::Usuario, proto};

pub async fn atualizar_categoria(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, uuid)): Path<(Uuid, Uuid)>,
    Extension(_): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::UpdateCategoriaRequest>,
) -> Result<Protobuf<proto::Categoria>, AppError> {

    let categoria = state.catalogo_service.atualizar_categoria(
        uuid,
        loja_uuid,
        p.nome,
        if p.descricao.is_empty() { None } else { Some(p.descricao) },
        if p.ordem == 0 { None } else { Some(p.ordem) },
        p.pizza_mode,
    ).await?;

    Ok(Protobuf(categoria.to_proto()))
}
