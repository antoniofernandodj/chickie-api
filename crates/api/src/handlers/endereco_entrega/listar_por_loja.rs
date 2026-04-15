use axum::{Extension, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    usecases::ListarEnderecosEntregaPorLojaUsecase,
    proto
};
use chickie_core::ports::to_proto::ToProto;
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn listar_por_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::ListarEnderecosResponse>, AppError> {

    let usecase = ListarEnderecosEntregaPorLojaUsecase::new(
        state.endereco_entrega_service.clone(),
        usuario,
        loja_uuid
    );

    let enderecos = usecase.executar().await?;

    let enderecos_proto = enderecos.iter().map(|e| e.to_proto()).collect();

    Ok(Protobuf(proto::ListarEnderecosResponse { enderecos: enderecos_proto }))
}
