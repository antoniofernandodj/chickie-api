use axum::{Router, routing::post, middleware::from_fn_with_state};
use std::sync::Arc;

use crate::handlers::{AppState, optional_auth_middleware, criar_pagamento, webhook_asaas};

pub fn pagamento_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    // Webhook é público (Asaas chama sem auth)
    let public = Router::new()
        .route("/webhook", post(webhook_asaas));

    // Criar pagamento: auth opcional (autenticado usa dados do usuário; anônimo passa pagador no body)
    let pagamento = Router::new()
        .route("/{pedido_uuid}", post(criar_pagamento))
        .layer(from_fn_with_state(s.clone(), optional_auth_middleware));

    Router::new()
        .merge(public)
        .merge(pagamento)
}
