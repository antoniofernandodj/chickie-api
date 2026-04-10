use axum::Router;
use axum::routing::get;

use crate::api::listar_usuarios;

pub fn usuario_routes() -> Router<std::sync::Arc<crate::api::AppState>> {
    Router::new()
        .route("/", get(listar_usuarios))
}
