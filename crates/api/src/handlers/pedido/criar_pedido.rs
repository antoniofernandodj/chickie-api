use axum::{
    Extension, Json, extract::State, response::IntoResponse, http::StatusCode
};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;
use rust_decimal::Decimal;

use chickie_core::{
    models::Usuario,
    usecases::{
        PedidoUsecase,
        ItemPedidoInput,
        ParteItemInput,
        EnderecoEntregaInput
    }
};
use crate::handlers::{dto::AppError, AppState};

#[derive(Deserialize)]
pub struct CriarPedidoRequest {
    pub loja_uuid: Uuid,
    pub taxa_entrega: Decimal,
    pub forma_pagamento: String,
    pub observacoes: Option<String>,
    pub codigo_cupom: Option<String>,
    pub itens: Vec<ItemPedidoRequest>,
    pub endereco_entrega: Option<EnderecoEntregaRequest>,
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
    usuario_ext: Option<Extension<Usuario>>,
    Json(p): Json<CriarPedidoRequest>,
) -> Result<impl IntoResponse, AppError> {

    let usuario = usuario_ext
        .map(|Extension(u)| u);

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

    let endereco = p.endereco_entrega.map(|e| EnderecoEntregaInput {
        cep: e.cep,
        logradouro: e.logradouro,
        numero: e.numero,
        complemento: e.complemento,
        bairro: e.bairro,
        cidade: e.cidade,
        estado: e.estado,
    });

    let pedido_uuid = usecase.criar_pedido(
        p.taxa_entrega,
        p.forma_pagamento,
        p.observacoes,
        p.codigo_cupom,
        itens,
        endereco,
    ).await?;

    Ok((StatusCode::CREATED, Json(serde_json::json!({ "uuid": pedido_uuid }))))
}
