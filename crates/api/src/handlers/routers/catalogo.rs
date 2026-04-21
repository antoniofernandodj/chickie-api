use axum::Router;
use axum::routing::{get, post, put, delete};

use crate::handlers::{
    criar_adicional,
    listar_adicionais,
    listar_adicionais_disponiveis,
    atualizar_disponibilidade,
    atualizar_adicional,
    deletar_adicional,
    criar_categoria,
    listar_categorias,
    atualizar_categoria,
    deletar_categoria,
    reordenar_categorias,
    listar_categorias_globais,
};

pub fn catalogo_routes() -> Router<std::sync::Arc<crate::handlers::AppState>> {
    Router::new()
        .route("/{loja_uuid}/adicionais", post(criar_adicional))
        .route("/{loja_uuid}/adicionais/{adicional_uuid}/disponibilidade", put(atualizar_disponibilidade))
        .route("/{loja_uuid}/adicionais/{adicional_uuid}", put(atualizar_adicional))
        .route("/{loja_uuid}/adicionais/{adicional_uuid}", delete(deletar_adicional))
        .route("/{loja_uuid}/categorias", post(criar_categoria))
        .route("/{loja_uuid}/categorias/reordenar", put(reordenar_categorias))
        .route("/{loja_uuid}/categorias/{uuid}", put(atualizar_categoria))
        .route("/{loja_uuid}/categorias/{uuid}", delete(deletar_categoria))
}

pub fn catalogo_public_routes() -> Router<std::sync::Arc<crate::handlers::AppState>> {
    Router::new()
        .route("/{loja_uuid}/adicionais", get(listar_adicionais))
        .route("/{loja_uuid}/adicionais/disponiveis", get(listar_adicionais_disponiveis))
        .route("/{loja_uuid}/categorias", get(listar_categorias))
        .route("/categorias/globais", get(listar_categorias_globais))
}
