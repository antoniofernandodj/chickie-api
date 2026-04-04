use axum::{
    Extension, Json, extract::State, response::IntoResponse, http::StatusCode
};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;
use rust_decimal::Decimal;

use crate::{
    api::{dto::AppError, AppState},
    models::Usuario,
    usecases::{
        PedidoUsecase,
        ItemPedidoInput,
        ParteItemInput,
        EnderecoEntregaInput
    }
};

#[derive(Deserialize)]
pub struct CriarPedidoRequest {
    pub loja_uuid: Uuid,
    pub taxa_entrega: Decimal,
    pub forma_pagamento: String,
    pub observacoes: Option<String>,
    pub codigo_cupom: Option<String>,
    pub itens: Vec<ItemPedidoRequest>,
    pub endereco_entrega: EnderecoEntregaRequest,
}

#[derive(Deserialize)]
pub struct ItemPedidoRequest {
    pub quantidade: i32,
    pub observacoes: Option<String>,
    pub partes: Vec<ParteItemRequest>,
}

#[derive(Deserialize)]
pub struct ParteItemRequest {
    pub produto_uuid: Uuid,
    pub posicao: i32,
}

#[derive(Deserialize)]
pub struct EnderecoEntregaRequest {
    pub cep: Option<String>,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
}

pub async fn criar_pedido(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<CriarPedidoRequest>,
) -> Result<impl IntoResponse, AppError> {

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        usuario,
        p.loja_uuid,
    );

    let itens: Vec<ItemPedidoInput> = p.itens.into_iter().map(|i| ItemPedidoInput {
        quantidade: i.quantidade,
        observacoes: i.observacoes,
        partes: i.partes.into_iter().map(|pp| ParteItemInput {
            produto_uuid: pp.produto_uuid,
            posicao: pp.posicao,
        }).collect(),
    }).collect();

    let pedido_uuid = usecase.criar_pedido(
        p.taxa_entrega,
        p.forma_pagamento,
        p.observacoes,
        p.codigo_cupom,
        itens,
        EnderecoEntregaInput {
            cep: p.endereco_entrega.cep,
            logradouro: p.endereco_entrega.logradouro,
            numero: p.endereco_entrega.numero,
            complemento: p.endereco_entrega.complemento,
            bairro: p.endereco_entrega.bairro,
            cidade: p.endereco_entrega.cidade,
            estado: p.endereco_entrega.estado,
        },
    ).await?;

    Ok((StatusCode::CREATED, Json(serde_json::json!({ "uuid": pedido_uuid }))))
}
