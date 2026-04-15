
use axum::{
    Extension, extract::State,
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

    let endereco = p.endereco_entrega
        .ok_or_else(|| AppError::BadRequest("endereco_entrega é obrigatório".into()))?;

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
            cep: if endereco.cep.is_empty() { None } else { Some(endereco.cep) },
            logradouro: endereco.logradouro,
            numero: endereco.numero,
            complemento: if endereco.complemento.is_empty() { None } else { Some(endereco.complemento) },
            bairro: endereco.bairro,
            cidade: endereco.cidade,
            estado: endereco.estado,
        },
    ).await?;

    Ok(Protobuf(proto::UuidResponse { uuid: pedido_uuid.to_string() }))
}