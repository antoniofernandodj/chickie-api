use std::sync::Arc;
use axum::{Json};
use axum::{Router, middleware::from_fn_with_state, response::IntoResponse, routing::{get, post, put, delete}};
use serde_json::json;

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
    criar_promocao,
    listar_cupons,
    atualizar_produto,
    wipe_database,
    avaliar_loja,
    avaliar_produto,
    adicionar_funcionario,
    adicionar_entregador,
    adicionar_cliente,
    listar_lojas_admin,
    criar_adicional,
    criar_categoria,
    listar_adicionais,
    listar_adicionais_disponiveis,
    marcar_indisponivel,
    criar_para_pedido,
    buscar_por_pedido,
    listar_por_loja,
    criar_endereco,
    listar_enderecos,
    buscar_endereco,
    atualizar_endereco,
    deletar_endereco,
    adicionar_favorita,
    remover_favorita,
    listar_minhas_favoritas,
    verificar_favorita
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
        .route("/", get(listar_lojas))
}

pub fn loja_admin_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(criar_loja))
        .route("/listar", get(listar_lojas_admin))
        .route("/{loja_uuid}/funcionarios", post(adicionar_funcionario))
        .route("/{loja_uuid}/entregadores", post(adicionar_entregador))
        .route("/{loja_uuid}/clientes", post(adicionar_cliente))
}

pub fn pedido_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/{loja_uuid}", post(criar_pedido))
            .layer(from_fn_with_state(s.clone(), auth_middleware))
        .route("/", get(listar_pedidos))
        .route("/{uuid}", get(buscar_pedido))
}


// Rotas de Catálogo
pub fn catalogo_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{loja_uuid}/adicionais", post(criar_adicional))
        .route("/{loja_uuid}/adicionais", get(listar_adicionais))
        .route("/{loja_uuid}/adicionais/disponiveis", get(listar_adicionais_disponiveis))
        .route("/{loja_uuid}/adicionais/{adicional_uuid}/indisponivel", put(marcar_indisponivel))
        .route("/{loja_uuid}/categorias", post(criar_categoria))
}

// Rotas de Endereço de Entrega
pub fn endereco_entrega_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{pedido_uuid}/{loja_uuid}", post(criar_para_pedido))
        .route("/{pedido_uuid}", get(buscar_por_pedido))
        .route("/{loja_uuid}/loja", get(listar_por_loja))
}

// Rotas de Endereço de Usuário
pub fn endereco_usuario_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(criar_endereco))
        .route("/", get(listar_enderecos))
        .route("/{uuid}", get(buscar_endereco))
        .route("/{uuid}", put(atualizar_endereco))
        .route("/{uuid}", delete(deletar_endereco))
}

// Rotas de Lojas Favoritas
pub fn loja_favorita_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{loja_uuid}", post(adicionar_favorita))
        .route("/{loja_uuid}", delete(remover_favorita))
        .route("/minhas", get(listar_minhas_favoritas))
        .route("/{loja_uuid}/verificar", get(verificar_favorita))
}
pub fn produto_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(criar_produto))
        .route("/", get(listar_produtos))
        .route("/{uuid}", put(atualizar_produto))
}

// Rotas de Marketing / Cupons / Avaliações
pub fn marketing_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(criar_cupom))
        .route("/", get(listar_cupons))
        .route("/{codigo}", get(validar_cupom))
        .route("/{loja_uuid}/avaliar-loja", post(avaliar_loja))
        .route("/{loja_uuid}/avaliar-produto", post(avaliar_produto))
        .route("/{loja_uuid}/promocoes", post(criar_promocao))
}


pub fn api_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    let mut router = Router::new()
        .nest("/pedidos", pedido_routes(&s))
        .nest("/usuarios", usuario_routes())
        .nest("/lojas", loja_routes())
        .nest("/produtos", produto_routes())
        .nest("/cupons", marketing_routes())
        .nest("/catalogo", catalogo_routes())
        .nest("/enderecos-entrega", endereco_entrega_routes())
        .nest("/enderecos-usuario", endereco_usuario_routes())
        .nest("/favoritos", loja_favorita_routes())
        .nest("/admin", loja_admin_routes())
            .layer(from_fn_with_state(s.clone(), auth_middleware))
        .nest("/auth", auth_routes())
        .route("/ok", get(ok_handler));

        let mode = std::env::var("MODE").unwrap_or_default();
        let is_dev = mode.eq_ignore_ascii_case("development");

        if is_dev {
            tracing::info!("🧹 MODE=DEVELOPMENT — registrando endpoint de limpar banco de dados");
            router = router.route("/wipe", delete(wipe_database))
        }

        router
}


pub async fn ok_handler() -> impl IntoResponse {
    Json(json!({"msg": "ok"}))
}