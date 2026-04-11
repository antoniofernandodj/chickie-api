use axum::Router;
use axum::routing::get;

use crate::api_handlers::listar_usuarios;

pub fn usuario_routes() -> Router<std::sync::Arc<crate::api_handlers::AppState>> {
    Router::new()
        .route("/", get(listar_usuarios))
}
