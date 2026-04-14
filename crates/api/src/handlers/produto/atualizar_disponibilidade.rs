use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    proto,
};

use crate::{
    handlers::{AppState, dto::AppError, protobuf::Protobuf},
};

pub async fn atualizar_disponibilidade_produto(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, produto_uuid)): Path<(Uuid, Uuid)>,
    Extension(_): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::AtualizarDisponibilidadeRequest>,
) -> Result<Protobuf<proto::GenericResponse>, AppError> {

    state.catalogo_service.atualizar_disponibilidade_produto(produto_uuid, loja_uuid, p.disponivel).await?;

    Ok(Protobuf(proto::GenericResponse {
        message: "Disponibilidade do produto atualizada com sucesso".to_string(),
        success: true,
    }))
}

/*

use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
};

use crate::{
    handlers::{AppState, dto::AppError},
};

#[derive(Deserialize)]
pub struct AtualizarDisponibilidadeProdutoRequest {
    pub disponivel: bool,
}

pub async fn atualizar_disponibilidade_produto(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, produto_uuid)): Path<(Uuid, Uuid)>,
    Extension(_): Extension<Usuario>,
    Json(p): Json<AtualizarDisponibilidadeProdutoRequest>,
) -> Result<impl IntoResponse, AppError> {

    state.catalogo_service.atualizar_disponibilidade_produto(produto_uuid, loja_uuid, p.disponivel).await?;

    Ok(StatusCode::NO_CONTENT)
}

*/
