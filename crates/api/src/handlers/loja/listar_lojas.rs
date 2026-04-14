use axum::{
    extract::{State},
};

use std::sync::Arc;
use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;
use chickie_core::{repositories::Repository, proto};

pub async fn listar_lojas(
    State(state): State<Arc<AppState>>
) -> Result<Protobuf<proto::ListarLojasResponse>, AppError> {

    let lojas = state
        .loja_repo
        .listar_todos()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?; 

    let proto_lojas: Vec<_> = lojas.into_iter()
        .map(|l| l.to_proto())
        .collect();

    Ok(Protobuf(proto::ListarLojasResponse {
        lojas: proto_lojas,
    }))
}


/*

use axum::{
    extract::{State},
    response::IntoResponse,
    Json
};


use std::sync::Arc;
use crate::handlers::dto::AppError;
use chickie_core::repositories::Repository;
use crate::handlers::AppState;


pub async fn listar_lojas(
    State(state): State<Arc<AppState>>
) -> Result<impl IntoResponse, AppError> {

    let lojas = state
        .loja_repo
        .listar_todos()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?; 

    Ok(Json(lojas))
}

*/