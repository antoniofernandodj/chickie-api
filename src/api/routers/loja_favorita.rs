use axum::Router;
use axum::routing::{get, post, delete};

use crate::api::{
    adicionar_favorita,
    remover_favorita,
    listar_minhas_favoritas,
    verificar_favorita,
};

pub fn loja_favorita_routes() -> Router<std::sync::Arc<crate::api::AppState>> {
    Router::new()
        .route("/{loja_uuid}", post(adicionar_favorita))
        .route("/{loja_uuid}", delete(remover_favorita))
        .route("/minhas", get(listar_minhas_favoritas))
        .route("/{loja_uuid}/verificar", get(verificar_favorita))
}
