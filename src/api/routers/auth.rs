use axum::{Router, middleware::from_fn_with_state, routing::{get, post}};
use std::sync::Arc;

use crate::api::{AppState, auth_middleware, me};
use crate::api::usuario::login;
use crate::api::{criar_usuario};

pub fn auth_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/me", get(me))
            .layer(from_fn_with_state(s.clone(), auth_middleware))
        .route("/login", post(login))
        .route("/signup", post(criar_usuario))
}
