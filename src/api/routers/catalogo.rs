use axum::Router;
use axum::routing::{get, post, put, delete};

use crate::api::{
    criar_adicional,
    listar_adicionais,
    listar_adicionais_disponiveis,
    marcar_indisponivel,
    criar_categoria,
    listar_categorias,
    atualizar_categoria,
    deletar_categoria,
};

pub fn catalogo_routes() -> Router<std::sync::Arc<crate::api::AppState>> {
    Router::new()
        .route("/{loja_uuid}/adicionais", post(criar_adicional))
        .route("/{loja_uuid}/adicionais", get(listar_adicionais))
        .route("/{loja_uuid}/adicionais/disponiveis", get(listar_adicionais_disponiveis))
        .route("/{loja_uuid}/adicionais/{adicional_uuid}/indisponivel", put(marcar_indisponivel))
        .route("/{loja_uuid}/categorias", post(criar_categoria))
        .route("/{loja_uuid}/categorias", get(listar_categorias))
        .route("/{loja_uuid}/categorias/{uuid}", put(atualizar_categoria))
        .route("/{loja_uuid}/categorias/{uuid}", delete(deletar_categoria))
}
