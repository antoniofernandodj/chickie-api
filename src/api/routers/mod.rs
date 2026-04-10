mod auth;
mod loja;
mod loja_admin;
mod pedido;
mod produto;
mod catalogo;
mod endereco_entrega;
mod endereco_usuario;
mod loja_favorita;
mod marketing;
mod ingrediente;
mod horario;
mod endereco_loja;
mod config_pedido;
mod cupom_admin;
mod funcionario;
mod entregador;
mod usuario;
mod wipe;

use std::sync::Arc;

use axum::{Router, middleware::from_fn_with_state, routing::get};

use crate::api::{AppState, auth_middleware};

pub use auth::auth_routes;
pub use loja::loja_routes;
pub use loja_admin::loja_admin_routes;
pub use pedido::pedido_routes;
pub use produto::produto_routes;
pub use catalogo::catalogo_routes;
pub use endereco_entrega::endereco_entrega_routes;
pub use endereco_usuario::endereco_usuario_routes;
pub use loja_favorita::loja_favorita_routes;
pub use marketing::marketing_routes;
pub use ingrediente::ingrediente_routes;
pub use horario::horario_routes;
pub use endereco_loja::endereco_loja_routes;
pub use config_pedido::config_pedido_routes;
pub use cupom_admin::cupom_admin_routes;
pub use funcionario::funcionario_routes;
pub use entregador::entregador_routes;
pub use usuario::usuario_routes;
pub use wipe::wipe_route;

use super::ok_handler;

pub fn api_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    let mut router = Router::new()
        .nest("/pedidos", pedido_routes(s))
        .nest("/usuarios", usuario_routes())
        .nest("/produtos", produto_routes())
        .nest("/marketing", marketing_routes(s))
        .nest("/catalogo", catalogo_routes())
        .nest("/enderecos-entrega", endereco_entrega_routes())
        .nest("/enderecos-usuario", endereco_usuario_routes())
        .nest("/favoritos", loja_favorita_routes())
        .nest("/ingredientes", ingrediente_routes())
        .nest("/horarios", horario_routes())
        .nest("/enderecos-loja", endereco_loja_routes())
        .nest("/config-pedido", config_pedido_routes())
        .nest("/cupons/admin", cupom_admin_routes())
        .nest("/funcionarios", funcionario_routes())
        .nest("/entregadores", entregador_routes())
        .nest("/admin", loja_admin_routes())
            .layer(from_fn_with_state(s.clone(), auth_middleware))
        .nest("/lojas", loja_routes())
        .nest("/auth", auth_routes(s))
        .route("/ok", get(ok_handler));

    let mode = std::env::var("MODE").unwrap_or_default();
    let is_dev = mode.eq_ignore_ascii_case("development");

    if is_dev {
        tracing::info!("🧹 MODE=DEVELOPMENT — registrando endpoint de limpar banco de dados");
        router = router.route("/wipe", wipe_route(s))
    }

    router
}
