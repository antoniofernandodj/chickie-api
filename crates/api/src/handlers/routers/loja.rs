use axum::{Router, routing::{get, patch, put, post}};

use crate::handlers::{
    listar_lojas,
    pesquisar_lojas,
    buscar_loja,
    buscar_loja_por_slug,
    verificar_slug_disponivel,
    marcar_loja_remocao,
    desmarcar_loja_remocao,
    alternar_loja_ativo,
    toggle_loja_bloqueado,
};

pub fn loja_routes() -> axum::Router<std::sync::Arc<crate::handlers::AppState>> {
    // let mut router = Router::new()
    //     .route("/", get(listar_lojas))
    //     .route("/pesquisar", get(pesquisar_lojas))
    //     .route("/{uuid}", get(buscar_loja))
    //     .route("/slug/{slug}", get(buscar_loja_por_slug))
    //     .route("/verificar-slug/{slug}", get(verificar_slug_disponivel))
    //     .route("/{loja_uuid}/marcar-remocao", patch(marcar_loja_remocao))
    //     .route("/{loja_uuid}/desmarcar-remocao", patch(desmarcar_loja_remocao))
    //     .route("/{loja_uuid}/ativo", put(alternar_loja_ativo))
    //     .route("/{loja_uuid}/bloqueado", post(toggle_loja_bloqueado))

    Router::new()
        .route("/", get(listar_lojas))
        .route("/pesquisar", get(pesquisar_lojas))
        .route("/{uuid}", get(buscar_loja))
        .route("/slug/{slug}", get(buscar_loja_por_slug))
        .route("/verificar-slug/{slug}", get(verificar_slug_disponivel))
        .route("/{loja_uuid}/marcar-remocao", patch(marcar_loja_remocao))
        .route("/{loja_uuid}/desmarcar-remocao", patch(desmarcar_loja_remocao))
        .route("/{loja_uuid}/ativo", put(alternar_loja_ativo))
        .route("/{loja_uuid}/bloqueado", post(toggle_loja_bloqueado))
}
