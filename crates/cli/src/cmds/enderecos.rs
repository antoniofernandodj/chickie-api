use chickie_core::models::EnderecoLoja;

use crate::app_state::AppState;
use crate::args::*;
use crate::helpers::{print_ok, print_err, json_print};

// ── Endereco de Entrega ──

pub async fn run_create_endereco_entrega(state: &AppState, args: CreateEnderecoEntregaArgs) {
    match state
        .endereco_entrega_service
        .criar_para_pedido(
            args.pedido_uuid,
            args.loja_uuid,
            args.cep,
            args.logradouro,
            args.numero,
            args.complemento,
            args.bairro,
            args.cidade,
            args.estado,
        )
        .await
    {
        Ok(e) => {
            print_ok("Endereço criado");
            json_print(&e);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_get_endereco_entrega(state: &AppState, args: GetEnderecoEntregaArgs) {
    match state
        .endereco_entrega_service
        .buscar_por_pedido(args.pedido_uuid)
        .await
    {
        Ok(Some(e)) => json_print(&e),
        Ok(None) => print_err("Endereço não encontrado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_enderecos_entrega(state: &AppState, args: ListEnderecosEntregaArgs) {
    match state
        .endereco_entrega_service
        .listar_por_loja(args.loja_uuid)
        .await
    {
        Ok(ends) => json_print(&ends),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

// ── Endereco de Loja ──

pub async fn run_list_enderecos_loja(state: &AppState, args: ListEnderecosLojaArgs) {
    match state
        .endereco_loja_service
        .listar_por_loja(args.loja_uuid)
        .await
    {
        Ok(ends) => json_print(&ends),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_create_endereco_loja(state: &AppState, args: CreateEnderecoLojaArgs) {
    let endereco = EnderecoLoja::new(
        args.loja_uuid,
        args.cep,
        args.logradouro,
        args.numero,
        args.complemento,
        args.bairro,
        args.cidade,
        args.estado,
        None,
        None,
    );

    match state.endereco_loja_service.criar(&endereco).await {
        Ok(_uuid) => {
            print_ok("Endereço de loja criado");
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_endereco_loja() {
    print_err("Update endereço de loja em construção");
}

pub async fn run_delete_endereco_loja() {
    print_err("Delete endereço de loja em construção");
}

// ── Endereco de Usuario ──

pub async fn run_create_endereco_usuario() {
    print_err("Create endereço de usuário em construção");
}

pub async fn run_list_enderecos_usuario(state: &AppState, args: ListEnderecosUsuarioArgs) {
    match state
        .endereco_usuario_service
        .listar_enderecos(args.usuario_uuid)
        .await
    {
        Ok(ends) => json_print(&ends),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_get_endereco_usuario() {
    // Need usuario_uuid - not available in this command args. Mark as TODO.
    print_err("GetEnderecoUsuario needs usuario_uuid — add it to args");
}

pub async fn run_update_endereco_usuario() {
    print_err("Update endereço de usuário em construção");
}

pub async fn run_delete_endereco_usuario() {
    // Need usuario_uuid - not available in this command args
    print_err("DeleteEnderecoUsuario needs usuario_uuid — add it to args");
}
