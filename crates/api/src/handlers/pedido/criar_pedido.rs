use axum::{
    Extension, extract::State, http::StatusCode
};
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;

use chickie_core::{
    models::Usuario,
    usecases::{
        PedidoUsecase,
        ItemPedidoInput,
        ParteItemInput,
        EnderecoEntregaInput
    },
    proto
};
use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};

pub async fn criar_pedido(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::CriarPedidoRequest>,
) -> Result<Protobuf<proto::UuidResponse>, AppError> {

    let loja_uuid = Uuid::parse_str(&p.loja_uuid)
        .map_err(|e| AppError::BadRequest(format!("Invalid loja_uuid: {}", e)))?;

    let taxa_entrega = Decimal::from_str_radix(&p.taxa_entrega, 10)
        .map_err(|e| AppError::BadRequest(format!("Invalid taxa_entrega: {}", e)))?;

    let itens: Vec<ItemPedidoInput> = p.itens.into_iter().map(|i| ItemPedidoInput {
        quantidade: i.quantidade,
        observacoes: if i.observacoes.is_empty() { None } else { Some(i.observacoes) },
        partes: i.partes.into_iter().map(|pp| ParteItemInput {
            produto_uuid: Uuid::parse_str(&pp.produto_uuid)
                .expect("Invalid produto_uuid"),
            posicao: pp.posicao,
        }).collect(),
    }).collect();

    let usecase = PedidoUsecase::new(
        state.pedido_service.clone(),
        Arc::clone(&state.produto_repo),
        usuario,
        loja_uuid,
    );

    let pedido_uuid = usecase.criar_pedido(
        taxa_entrega,
        p.forma_pagamento,
        if p.observacoes.is_empty() { None } else { Some(p.observacoes) },
        if p.codigo_cupom.is_empty() { None } else { Some(p.codigo_cupom) },
        itens,
        EnderecoEntregaInput {
            cep: if p.endereco_entrega.cep.is_empty() { None } else { Some(p.endereco_entrega.cep) },
            logradouro: p.endereco_entrega.logradouro,
            numero: p.endereco_entrega.numero,
            complemento: if p.endereco_entrega.complemento.is_empty() { None } else { Some(p.endereco_entrega.complemento) },
            bairro: p.endereco_entrega.bairro,
            cidade: p.endereco_entrega.cidade,
            estado: p.endereco_entrega.estado,
        },
    ).await?;

    Ok(Protobuf(proto::UuidResponse {
        uuid: pedido_uuid.to_string(),
    }))
}

/*
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
*/
