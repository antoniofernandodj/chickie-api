use axum::{Router, routing::get};

use crate::api::{
    listar_lojas,
    pesquisar_lojas,
    buscar_loja,
    buscar_loja_por_slug,
    verificar_slug_disponivel,
};

pub fn loja_routes() -> axum::Router<std::sync::Arc<crate::api::AppState>> {
    Router::new()
        .route("/", get(listar_lojas))
        .route("/pesquisar", get(pesquisar_lojas))
        .route("/{uuid}", get(buscar_loja))
        .route("/slug/{slug}", get(buscar_loja_por_slug))
        .route("/verificar-slug/{slug}", get(verificar_slug_disponivel))
}
