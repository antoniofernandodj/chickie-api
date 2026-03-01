use axum::{
    Json, extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}
};
use uuid::Uuid;

use std::sync::Arc;
use crate::{api::CreatePedidoRequest, models::{ParteDeItemPedido, Pedido}};
use crate::repositories::Repository;
use crate::api::AppState;



pub async fn criar_pedido(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Json(payload): Json<CreatePedidoRequest>,
) -> Response {
    
    // 1. Buscar produtos para montar as partes (validação básica)
    let mut partes_pedido: Vec<ParteDeItemPedido> = Vec::new();
    
    // Nota: Em produção, isso deveria ser otimizado (buscar todos produtos de uma vez)
    for item_req in &payload.itens {
        for parte_req in &item_req.partes {
            let produto = match state
                .produto_repo
                .buscar_por_uuid(parte_req.produto_uuid)
                .await {
                    Ok(Some(p)) => p,
                    Ok(None) => return (StatusCode::NOT_FOUND, "Produto não encontrado").into_response(),
                    Err(e) => return (StatusCode::BAD_REQUEST, format!("Erro ao buscar produto: {}", e)).into_response(),
                };


            partes_pedido.push(ParteDeItemPedido::new(&produto, parte_req.posicao));
        }
    }

    // 2. Criar struct Pedido
    let mut pedido = Pedido::new(
        payload.usuario_uuid,
        payload.loja_uuid,
        0.0, // Subtotal calculado depois
        payload.taxa_entrega,
        payload.forma_pagamento,
        payload.observacoes,
    );

    // 3. Adicionar itens ao pedido
    // Nota: A lógica atual do seu modelo adiciona todas as partes de uma vez em um item.
    // Aqui estou simplificando para adicionar os itens do payload.
    let mut partes_iter = partes_pedido.into_iter();
    for item_req in payload.itens {
        let mut partes_item: Vec<ParteDeItemPedido> = Vec::new();
        for _ in 0..item_req.partes.len() {
            if let Some(p) = partes_iter.next() {
                partes_item.push(p);
            }
        }
        pedido.adicionar_item(item_req.quantidade, item_req.observacoes, partes_item);
    }

    // 4. Processar e Salvar
    if let Err(e) = state
        .pedido_service
        .processar_e_finalizar_pedido(
            &mut pedido,
            payload.codigo_cupom
        ).await {
            return (StatusCode::BAD_REQUEST, e).into_response();
    }
    
    if let Err(e) = state
        .pedido_service
        .salvar(&pedido)
        .await {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    Json(pedido).into_response()

}
