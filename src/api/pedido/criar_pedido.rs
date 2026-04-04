use axum::{
    Extension, Json, extract::{Path, State}, response::{IntoResponse, Response}
};
use serde_json::json;
use uuid::Uuid;

use std::sync::Arc;
use crate::{api::{CreatePedidoRequest, dto::AppError}, models::{ParteDeItemPedido, Pedido, Usuario}};
use crate::repositories::Repository;
use crate::api::AppState;



pub async fn criar_pedido(
    State(state): State<Arc<AppState>>,
    Extension(usuario_logado): Extension<Usuario>,
    Path(loja_uuid): Path<Uuid>,
    Json(payload): Json<CreatePedidoRequest>,
) -> Response {


    // 2. Buscar produtos e montar as partes do pedido (validação de existência)
    let mut partes_por_item: Vec<Vec<ParteDeItemPedido>> = Vec::new();
    
    for item_req in &payload.itens {
        let mut partes_item = Vec::new();
        for parte_req in &item_req.partes {
            let produto = match state
                .produto_repo
                .buscar_por_uuid(parte_req.produto_uuid)
                .await {
                    Ok(Some(p)) => p,
                    Ok(None) => return AppError::NotFound(
                        format!("Produto {} não encontrado", parte_req.produto_uuid)
                    ).into_response(),
                    Err(e) => return AppError::BadRequest(
                        format!("Erro ao buscar produto: {}", e)
                    ).into_response(),
                };

            partes_item.push(ParteDeItemPedido::new(&produto, parte_req.posicao));
        }
        partes_por_item.push(partes_item);
    }

    // 3. Criar struct Pedido base
    let mut pedido = Pedido::new(
        usuario_logado.uuid,
        loja_uuid,
        0.0, // subtotal calculado pelo service
        payload.taxa_entrega,
        payload.forma_pagamento,
        payload.observacoes,
    );

    // 4. Adicionar itens ao pedido com suas partes
    let mut partes_iter = partes_por_item.into_iter();
    for item_req in payload.itens {
        if let Some(partes_item) = partes_iter.next() {
            pedido.adicionar_item(
                item_req.quantidade, 
                item_req.observacoes, 
                partes_item
            );
        }
    }

    // 5. Extrair e converter endereço de entrega para o modelo de domínio
    let endereco_entrega: crate::models::EnderecoEntrega = payload
        .endereco_entrega
        .to_endereco_entrega(Uuid::nil(), loja_uuid); // UUID temporário, será substituído

    // 6. Chamar o service method unificado (processa + salva pedido + cria endereço)
    match state
        .pedido_service
        .criar_pedido_com_entrega(
            &mut pedido,
            endereco_entrega,
            payload.codigo_cupom,
        )
        .await {
            Ok(pedido_uuid) => {
                // Retorna apenas o UUID ou o pedido completo, conforme necessidade
                Json(json!({ "uuid": pedido_uuid })).into_response()
            },
            Err(e) => AppError::BadRequest(e).into_response(),
    }
}