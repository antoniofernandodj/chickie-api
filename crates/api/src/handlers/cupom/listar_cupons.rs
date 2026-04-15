use axum::{Extension, extract::{Path, State}};
use uuid::Uuid;
use std::sync::Arc;

use chickie_core::{
    models::Usuario,
    usecases::MarketingUsecase,
    proto
};
use chickie_core::ports::to_proto::ToProto;
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn listar_cupons(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::ListarCuponsResponse>, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    let cupons = usecase.listar_cupons().await?;

    let cupons_proto = cupons.iter().map(|c| c.to_proto()).collect();

    Ok(Protobuf(proto::ListarCuponsResponse { cupons: cupons_proto }))
}
