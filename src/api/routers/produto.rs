use axum::Router;
use axum::routing::{get, post, put, delete};

use crate::api::{
    criar_produto,
    listar_produtos,
    listar_produtos_por_categoria,
    buscar_produto_por_uuid,
    atualizar_produto,
    deletar_produto,
    subir_imagem_produto,
};

pub fn produto_routes() -> Router<std::sync::Arc<crate::api::AppState>> {
    Router::new()
        .route("/", post(criar_produto))
        .route("/listar/{loja_uuid}", get(listar_produtos))
        .route("/categoria/{categoria_uuid}", get(listar_produtos_por_categoria))
        .route("/{uuid}", get(buscar_produto_por_uuid))
        .route("/{uuid}", put(atualizar_produto))
        .route("/{uuid}", delete(deletar_produto))
        .route("/{uuid}/imagem", post(subir_imagem_produto))
}
