use axum::{
    Extension, extract::{Path, State},
};
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::Usuario,
    ports::to_proto::ToProto,
    usecases::MarketingUsecase,
    proto,
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn listar_promocoes(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::ListarPromocoesResponse>, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    let promocoes = usecase.listar_promocoes().await?;

    let proto_promocoes: Vec<_> = promocoes.into_iter()
        .map(|p| p.to_proto())
        .collect();

    Ok(Protobuf(proto::ListarPromocoesResponse {
        promocoes: proto_promocoes,
    }))
}
