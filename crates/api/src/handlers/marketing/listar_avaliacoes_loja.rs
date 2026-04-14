use std::sync::Arc;
use axum::{
    extract::{Path, State, Extension},
};
use uuid::Uuid;
use chickie_core::{
    models::Usuario,
    ports::to_proto::ToProto,
    usecases::MarketingUsecase,
    proto,
};
use crate::handlers::{AppState, dto::AppError, protobuf::Protobuf};

pub async fn listar_avaliacoes_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::ListarAvaliacoesLojaResponse>, AppError> {
    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );
    let avaliacoes = usecase.listar_avaliacoes_loja().await?;
    
    let proto_avaliacoes: Vec<_> = avaliacoes.into_iter()
        .map(|a| a.to_proto())
        .collect();

    Ok(Protobuf(proto::ListarAvaliacoesLojaResponse {
        avaliacoes: proto_avaliacoes,
    }))
}
