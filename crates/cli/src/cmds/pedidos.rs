use chickie_core::models::{Pedido, EstadoDePedido};

use crate::app_state::AppState;
use crate::args::*;
use crate::helpers::{print_ok, print_err, json_print, parse_decimal};

pub async fn run_create_pedido(state: &AppState, args: CreatePedidoArgs) {
    let subtotal = parse_decimal(args.subtotal);
    let taxa = parse_decimal(args.taxa_entrega);
    let pedido = Pedido::new(
        Some(args.usuario_uuid),
        args.loja_uuid,
        subtotal,
        taxa,
        args.forma_pagamento,
        args.observacoes,
    );

    // TODO: cupom handling - not supported via this CLI path yet
    if args.cupom.is_some() {
        print_err("Cupom not supported via CLI yet");
    }

    match state
        .pedido_service
        .salvar(&pedido)
        .await
    {
        Ok(uuid) => {
            print_ok("Pedido criado");
            println!("{}", uuid);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_pedidos() {
    print_err("ListPedidos precisa de loja_uuid — use ListPedidosPorLoja");
}

pub async fn run_list_meus_pedidos(state: &AppState, args: ListMeusPedidosArgs) {
    match state
        .pedido_service
        .listar_por_usuario(args.usuario_uuid)
        .await
    {
        Ok(pedidos) => json_print(&pedidos),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_pedidos_por_loja(state: &AppState, args: ListPedidosPorLojaArgs) {
    match state.pedido_service.listar_por_loja(args.loja_uuid).await {
        Ok(pedidos) => json_print(&pedidos),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_get_pedido() {
    // buscar_completo needs loja_uuid too
    print_err("GetPedido needs loja_uuid — add it to args");
}

pub async fn run_get_pedido_com_entrega(state: &AppState, args: GetPedidoComEntregaArgs) {
    match state
        .pedido_service
        .buscar_pedido_com_entrega(args.uuid, uuid::Uuid::nil())
        .await
    {
        Ok(p) => {
            // Serialize manually since PedidoComEntrega doesn't implement Serialize
            println!("Pedido UUID: {}", p.pedido.uuid);
            println!("Status: {:?}", p.pedido.status);
            println!("Total: {}", p.pedido.total);
            if let Some(ref endereco) = p.endereco_entrega {
                println!("Endereço: {}, {} - {}, {}", endereco.logradouro, endereco.numero, endereco.bairro, endereco.cidade);
            } else {
                println!("Endereço: não informado");
            }
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_pedido_status(state: &AppState, args: UpdatePedidoStatusArgs) {
    let status = EstadoDePedido::from_str(&args.novo_status);
    match status {
        Ok(s) => {
            match state
                .pedido_service
                .atualizar_status(args.uuid, s)
                .await
            {
                Ok(p) => {
                    print_ok("Status atualizado");
                    json_print(&p);
                }
                Err(e) => print_err(&format!("{:?}", e)),
            }
        }
        Err(e) => print_err(&e),
    }
}

pub async fn run_atribuir_entregador(state: &AppState, args: AtribuirEntregadorArgs) {
    match state
        .pedido_service
        .atribuir_entregador(args.pedido_uuid, args.entregador_uuid)
        .await
    {
        Ok(()) => print_ok("Entregador atribuído"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_remover_entregador(state: &AppState, args: RemoverEntregadorArgs) {
    match state
        .pedido_service
        .remover_entregador(args.pedido_uuid)
        .await
    {
        Ok(()) => print_ok("Entregador removido do pedido"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}
