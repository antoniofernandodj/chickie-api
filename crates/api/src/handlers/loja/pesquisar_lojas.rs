use axum::{
    extract::{Query, State}
};
use serde::Deserialize;
use std::sync::Arc;

use chickie_core::usecases::LojaUsecase;
use chickie_core::proto;
use chickie_core::ports::to_proto::ToProto;
use crate::handlers::dto::AppError;
use crate::handlers::AppState;
use crate::handlers::protobuf::Protobuf;

#[derive(Deserialize)]
pub struct PesquisaQuery {
    pub termo: String,
}

pub async fn pesquisar_lojas(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PesquisaQuery>,
) -> Result<Protobuf<proto::ListarLojasResponse>, AppError> {

    let usecase = LojaUsecase::new(state.loja_service.clone());

    let lojas = usecase
        .pesquisar_lojas(&query.termo)
        .await
        .map_err(|e| AppError::Internal(e))?;

    let lojas_proto = lojas.iter().map(|l| l.to_proto()).collect();

    Ok(Protobuf(proto::ListarLojasResponse { lojas: lojas_proto }))
}
