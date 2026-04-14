use std::sync::Arc;

use axum::extract::{Path, State, Extension};
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    usecases::VerificarLojaFavoritaUsecase,
    proto,
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn verificar_favorita(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::DisponibilidadeResponse>, AppError> {

    let usecase = VerificarLojaFavoritaUsecase::new(
        state.loja_favorita_service.clone(),
        usuario,
        loja_uuid
    );

    let favorita = usecase.executar().await?;

    Ok(Protobuf(proto::DisponibilidadeResponse {
        disponivel: favorita,
    }))
}
