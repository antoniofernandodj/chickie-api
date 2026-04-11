use std::sync::Arc;

use axum::{
    Json, extract::{Path, State, Extension}, response::IntoResponse
};
use uuid::Uuid;
use chickie_core::{
    models::Usuario,
    usecases::MarketingUsecase
};
use crate::api_handlers::{
    dto::{AppError, AvaliarProdutoRequest},
    AppState
};

pub async fn avaliar_produto(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Json(payload): Json<AvaliarProdutoRequest>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = MarketingUsecase::new(
        state.marketing_service.clone(),
        loja_uuid,
        usuario
    );

    let avaliacao = usecase.avaliar_produto(
        payload.produto_uuid,
        payload.nota,
        payload.descricao,
        payload.comentario
    ).await?;

    Ok(Json(avaliacao))
}
