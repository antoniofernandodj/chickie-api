use axum::Router;
use axum::routing::{get, post};

use crate::api_handlers::{
    criar_loja,
    listar_lojas_admin,
    listar_minhas_lojas,
    adicionar_funcionario,
    adicionar_entregador,
    adicionar_cliente,
};

pub fn loja_admin_routes() -> Router<std::sync::Arc<crate::api_handlers::AppState>> {
    Router::new()
        .route("/lojas", post(criar_loja))
        .route("/lojas/listar", get(listar_lojas_admin))
        .route("/lojas/minhas-lojas", get(listar_minhas_lojas))
        .route("/lojas/{loja_uuid}/funcionarios", post(adicionar_funcionario))
        .route("/lojas/{loja_uuid}/entregadores", post(adicionar_entregador))
        .route("/lojas/{loja_uuid}/clientes", post(adicionar_cliente))
}
