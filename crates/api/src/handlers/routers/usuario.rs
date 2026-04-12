use axum::Router;
use axum::routing::get;

use crate::handlers::listar_usuarios;

pub fn usuario_routes() -> Router<std::sync::Arc<crate::handlers::AppState>> {
    Router::new()
        .route("/", get(listar_usuarios))
}
