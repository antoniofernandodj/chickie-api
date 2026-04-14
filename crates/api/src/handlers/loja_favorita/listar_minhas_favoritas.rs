use std::sync::Arc;

use axum::extract::{State, Extension};

use chickie_core::{
    models::Usuario,
    ports::to_proto::ToProto,
    usecases::ListarLojasFavoritasUsecase,
    proto,
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn listar_minhas_favoritas(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::ListarLojasFavoritasResponse>, AppError> {

    let usecase = ListarLojasFavoritasUsecase::new(
        state.loja_favorita_service.clone(),
        usuario
    );

    let favoritas = usecase.executar().await?;

    Ok(Protobuf(proto::ListarLojasFavoritasResponse {
        favoritas: favoritas.into_iter().map(|f| f.to_proto()).collect(),
    }))
}
