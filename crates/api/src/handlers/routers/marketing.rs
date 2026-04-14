use axum::{Router, middleware::from_fn_with_state, routing::{get, post, put, delete}};
use std::sync::Arc;

use crate::handlers::{AppState, auth_middleware};
use crate::handlers::{
    criar_cupom,
    validar_cupom,
    listar_cupons,
    avaliar_loja,
    avaliar_produto,
    criar_promocao,
    listar_promocoes,
    atualizar_promocao,
    deletar_promocao,
    listar_avaliacoes_loja,
    buscar_avaliacao_loja,
    atualizar_avaliacao_loja,
    deletar_avaliacao_loja,
    listar_avaliacoes_produto_por_loja,
    listar_avaliacoes_produto_por_produto,
    buscar_avaliacao_produto,
    atualizar_avaliacao_produto,
    deletar_avaliacao_produto,
};

pub fn marketing_routes(s: &Arc<AppState>) -> Router<Arc<AppState>> {
    // let mut router = Router::new()
    //     // Cupons
    //     .route("/{loja_uuid}/cupons", post(criar_cupom))
    //     .route("/cupons/{codigo}", get(validar_cupom))
    //         .layer(from_fn_with_state(s.clone(), auth_middleware))
    //     .route("/cupons", get(listar_cupons))

    //     // Avaliacoes de Loja
    //     .route("/{loja_uuid}/avaliacoes-loja", get(listar_avaliacoes_loja))
    //     .route("/avaliacoes-loja/{uuid}", get(buscar_avaliacao_loja))
    //     .route("/avaliacoes-loja/{uuid}", put(atualizar_avaliacao_loja))
    //     .route("/avaliacoes-loja/{uuid}", delete(deletar_avaliacao_loja))
    //     .route("/{loja_uuid}/avaliar-loja", post(avaliar_loja))

    //     // Avaliacoes de Produto
    //     .route("/{loja_uuid}/avaliacoes-produto", get(listar_avaliacoes_produto_por_loja))
    //     .route("/avaliacoes-produto/produto/{produto_uuid}", get(listar_avaliacoes_produto_por_produto))
    //     .route("/avaliacoes-produto/{uuid}", get(buscar_avaliacao_produto))
    //     .route("/avaliacoes-produto/{uuid}", put(atualizar_avaliacao_produto))
    //     .route("/avaliacoes-produto/{uuid}", delete(deletar_avaliacao_produto))
    //     .route("/{loja_uuid}/avaliar-produto", post(avaliar_produto))

    //     // Promocoes
    //     .route("/{loja_uuid}/promocoes", post(criar_promocao))
    //     .route("/{loja_uuid}/promocoes", get(listar_promocoes))
    //     .route("/{loja_uuid}/promocoes/{uuid}", put(atualizar_promocao))
    //     .route("/{loja_uuid}/promocoes/{uuid}", delete(deletar_promocao))

    Router::new()
        // Cupons
        .route("/{loja_uuid}/cupons", post(criar_cupom))
        .route("/cupons/{codigo}", get(validar_cupom))
            .layer(from_fn_with_state(s.clone(), auth_middleware))
        .route("/cupons", get(listar_cupons))

        // Avaliacoes de Loja
        .route("/{loja_uuid}/avaliacoes-loja", get(listar_avaliacoes_loja))
        .route("/avaliacoes-loja/{uuid}", get(buscar_avaliacao_loja))
        .route("/avaliacoes-loja/{uuid}", put(atualizar_avaliacao_loja))
        .route("/avaliacoes-loja/{uuid}", delete(deletar_avaliacao_loja))
        .route("/{loja_uuid}/avaliar-loja", post(avaliar_loja))

        // Avaliacoes de Produto
        .route("/{loja_uuid}/avaliacoes-produto", get(listar_avaliacoes_produto_por_loja))
        .route("/avaliacoes-produto/produto/{produto_uuid}", get(listar_avaliacoes_produto_por_produto))
        .route("/avaliacoes-produto/{uuid}", get(buscar_avaliacao_produto))
        .route("/avaliacoes-produto/{uuid}", put(atualizar_avaliacao_produto))
        .route("/avaliacoes-produto/{uuid}", delete(deletar_avaliacao_produto))
        .route("/{loja_uuid}/avaliar-produto", post(avaliar_produto))

        // Promocoes
        .route("/{loja_uuid}/promocoes", post(criar_promocao))
        .route("/{loja_uuid}/promocoes", get(listar_promocoes))
        .route("/{loja_uuid}/promocoes/{uuid}", put(atualizar_promocao))
        .route("/{loja_uuid}/promocoes/{uuid}", delete(deletar_promocao))
}
