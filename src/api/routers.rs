use std::sync::Arc;

use axum::{Router, middleware::from_fn_with_state, routing::{get, post, put}};

use crate::api::{AppState, auth_middleware};
use crate::api::{
    usuario::login
};
use crate::api::{
    buscar_pedido,
    criar_loja,
    criar_pedido,
    criar_usuario,
    listar_lojas,
    listar_pedidos,
    listar_usuarios,
    criar_produto,
    listar_produtos,
    criar_cupom,
    validar_cupom,
    atualizar_produto
};


pub fn usuario_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(listar_usuarios))
}

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/signup", post(criar_usuario))
        .route("/login", post(login))
}

pub fn loja_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(criar_loja))
        .route("/", get(listar_lojas))
}

pub fn pedido_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(criar_pedido))
        .route("/", get(listar_pedidos))
        .route("/{uuid}", get(buscar_pedido))
}


// Rotas de Catálogo / Produtos
pub fn produto_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(criar_produto))
        .route("/", get(listar_produtos))
        .route("/{uuid}", put(atualizar_produto))
}

// Rotas de Marketing / Cupons
pub fn marketing_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(criar_cupom))
        .route("/{codigo}", get(validar_cupom))
}


pub fn api_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .nest("/pedidos", pedido_routes())
        .nest("/usuarios", usuario_routes())
        .nest("/lojas", loja_routes())
        .nest("/produtos", produto_routes())
        .nest("/cupons", marketing_routes())
            .layer(from_fn_with_state(s.clone(), auth_middleware))
        .nest("/auth", auth_routes())
}
