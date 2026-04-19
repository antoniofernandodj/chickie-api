use axum::{
    Extension, Json, extract::State, response::IntoResponse
};
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    usecases::PedidoUsecase
};
use crate::handlers::{dto::AppError, AppState};

/// GET /api/pedidos/meus
/// Lista todos os pedidos do usuário autenticado.
pub async fn listar_meus_pedidos(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
) -> Result<impl IntoResponse, AppError> {

    // Usa uma loja_uuid fictícia (nil) pois o usecase só precisa do
    // usuario.uuid para listar_por_usuario. O campo loja_uuid do usecase
    // é ignorado neste método.
    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        Some(usuario),
        Uuid::nil(),
    );

    let pedidos = usecase
        .listar_por_usuario()
        .await?;

    Ok(Json(pedidos))
}
