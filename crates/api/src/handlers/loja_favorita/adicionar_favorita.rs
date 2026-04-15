use std::sync::Arc;

use axum::{extract::{Path, State, Extension}};
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    usecases::AdicionarLojaFavoritaUsecase,
    proto
};
use chickie_core::ports::to_proto::ToProto;
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn adicionar_favorita(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Protobuf<proto::LojaFavorita>, AppError> {

    let usecase = AdicionarLojaFavoritaUsecase::new(
        state.loja_favorita_service.clone(),
        usuario,
        loja_uuid
    );

    let favorita = usecase.executar().await?;

    Ok(Protobuf(favorita.to_proto()))
}
