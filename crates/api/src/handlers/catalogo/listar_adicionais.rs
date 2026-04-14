use std::sync::Arc;

use axum::{Extension, extract::{Path, State}};
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    ports::to_proto::ToProto,
    proto,
    usecases::CatalogoUsecase
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn listar_adicionais(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::ListarAdicionaisResponse>, AppError> {

    let usecase = CatalogoUsecase::new(
        state.catalogo_service.clone(),
        loja_uuid,
        usuario
    );

    let adicionais = usecase.listar_adicionais().await?;

    Ok(Protobuf(proto::ListarAdicionaisResponse {
        adicionais: adicionais.into_iter().map(|a| a.to_proto()).collect(),
    }))
}
