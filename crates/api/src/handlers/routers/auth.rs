use axum::{Router, middleware::from_fn_with_state, routing::{get, post}};
use std::sync::Arc;

use crate::handlers::{
    AppState, auth_middleware, me,
    verificar_email, verificar_username, verificar_celular,
    criar_usuario, confirmar_cadastro,
};
use crate::handlers::usuario::login;

pub fn auth_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/me", get(me))
            .layer(from_fn_with_state(s.clone(), auth_middleware))
        .route("/login", post(login))
        .route("/signup", post(criar_usuario))
        .route("/confirmar-email", get(confirmar_cadastro))
        .route("/verificar-email", post(verificar_email))
        .route("/verificar-username", post(verificar_username))
        .route("/verificar-celular", post(verificar_celular))
}
