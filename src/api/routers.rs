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
    buscar_loja,
    listar_pedidos,
    // processar_e_exibir_precos,
    listar_por_loja,
    buscar_pedido_com_entrega,
    atualizar_status,
    criar_ingrediente,
    listar_ingredientes,
    atualizar_ingrediente,
    deletar_ingrediente,
    listar_horarios,
    criar_ou_atualizar_horario,
    definir_ativo,
    deletar_horario_dia,
    buscar_config_pedido,
    salvar_config_pedido,
    atualizar_cupom,
    deletar_cupom,
    atualizar_funcionario,
    funcionario_trocar_email_senha,
    atualizar_entregador,
    entregador_trocar_email_senha,
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
    listar_promocoes,
    atualizar_promocao,
    deletar_promocao,
    adicionar_funcionario,
    adicionar_entregador,
    adicionar_cliente,
    listar_lojas_admin,
    listar_minhas_lojas,
    pesquisar_lojas,
    buscar_loja_por_slug,
    criar_adicional,
    criar_categoria,
    listar_adicionais,
    listar_adicionais_disponiveis,
    marcar_indisponivel,
    criar_para_pedido,
    buscar_por_pedido,
    listar_enderecos_por_loja,
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
        .route("/pesquisar", get(pesquisar_lojas))
        .route("/{uuid}", get(buscar_loja))
        .route("/slug/{slug}", get(buscar_loja_por_slug))
}

pub fn loja_admin_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/lojas", post(criar_loja))
        .route("/listar", get(listar_lojas_admin))
        .route("/minhas-lojas", get(listar_minhas_lojas))
        .route("/{loja_uuid}/funcionarios", post(adicionar_funcionario))
        .route("/{loja_uuid}/entregadores", post(adicionar_entregador))
        .route("/{loja_uuid}/clientes", post(adicionar_cliente))
}

pub fn pedido_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/criar", post(criar_pedido))
        .route("/listar", get(listar_pedidos))
        .route("/por-loja/{loja_uuid}", get(listar_por_loja))
        .route("/{uuid}", get(buscar_pedido))
        .route("/{uuid}/com-entrega", get(buscar_pedido_com_entrega))
        .route("/{uuid}/status", put(atualizar_status))
        .layer(from_fn_with_state(s.clone(), auth_middleware))
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
        .route("/{loja_uuid}/loja", get(listar_enderecos_por_loja))
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

pub fn horario_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{loja_uuid}", get(listar_horarios))
        .route("/{loja_uuid}", post(criar_ou_atualizar_horario))
        .route("/{loja_uuid}/dia/{dia_semana}/ativo", put(definir_ativo))
        .route("/{loja_uuid}/dia/{dia_semana}", delete(deletar_horario_dia))
}

pub fn config_pedido_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{loja_uuid}", get(buscar_config_pedido))
        .route("/{loja_uuid}", put(salvar_config_pedido))
}

pub fn cupom_admin_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{loja_uuid}/{uuid}", put(atualizar_cupom))
        .route("/{loja_uuid}/{uuid}", delete(deletar_cupom))
}

pub fn ingrediente_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{loja_uuid}", post(criar_ingrediente))
        .route("/{loja_uuid}", get(listar_ingredientes))
        .route("/{loja_uuid}/{uuid}", put(atualizar_ingrediente))
        .route("/{loja_uuid}/{uuid}", delete(deletar_ingrediente))
}

pub fn funcionario_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{loja_uuid}/{uuid}", put(atualizar_funcionario))
        .route("/{loja_uuid}/usuarios/{usuario_uuid}/credenciais", put(funcionario_trocar_email_senha))
}

pub fn entregador_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{loja_uuid}/{uuid}", put(atualizar_entregador))
        .route("/{loja_uuid}/usuarios/{usuario_uuid}/credenciais", put(entregador_trocar_email_senha))
}

pub fn produto_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(criar_produto))
        .route("/", get(listar_produtos))
        .route("/{uuid}", put(atualizar_produto))
}

// Rotas de Marketing / Cupons / Avaliações
pub fn marketing_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/{loja_uuid}/cupons", post(criar_cupom))
        .route("/cupons/{codigo}", get(validar_cupom))
            .layer(from_fn_with_state(s.clone(), auth_middleware))
        .route("/cupons", get(listar_cupons))
        .route("/{loja_uuid}/avaliar-loja", post(avaliar_loja))
        .route("/{loja_uuid}/avaliar-produto", post(avaliar_produto))
        .route("/{loja_uuid}/promocoes", post(criar_promocao))
        .route("/{loja_uuid}/promocoes", get(listar_promocoes))
        .route("/{loja_uuid}/promocoes/{uuid}", put(atualizar_promocao))
        .route("/{loja_uuid}/promocoes/{uuid}", delete(deletar_promocao))
}

pub fn api_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    let mut router = Router::new()
        .nest("/pedidos", pedido_routes(&s))
        .nest("/usuarios", usuario_routes())
        .nest("/produtos", produto_routes())
        .nest("/marketing", marketing_routes(&s))
        .nest("/catalogo", catalogo_routes())
        .nest("/enderecos-entrega", endereco_entrega_routes())
        .nest("/enderecos-usuario", endereco_usuario_routes())
        .nest("/favoritos", loja_favorita_routes())
        .nest("/ingredientes", ingrediente_routes())
        .nest("/horarios", horario_routes())
        .nest("/config-pedido", config_pedido_routes())
        .nest("/cupons/admin", cupom_admin_routes())
        .nest("/funcionarios", funcionario_routes())
        .nest("/entregadores", entregador_routes())
        .nest("/admin", loja_admin_routes())
            .layer(from_fn_with_state(s.clone(), auth_middleware))
        .nest("/lojas", loja_routes())
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
