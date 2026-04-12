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
        Ok(uuid) => {
            print_ok("Endereço de loja criado");
            json_print(&EnderecoLoja {
                uuid,
                loja_uuid: endereco.loja_uuid,
                cep: endereco.cep,
                logradouro: endereco.logradouro.clone(),
                numero: endereco.numero.clone(),
                complemento: endereco.complemento.clone(),
                bairro: endereco.bairro.clone(),
                cidade: endereco.cidade.clone(),
                estado: endereco.estado.clone(),
                latitude: endereco.latitude,
                longitude: endereco.longitude,
            });
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_endereco_loja(state: &AppState, args: UpdateEnderecoLojaArgs) {
    // Fetch existing address first to get current values, then merge with provided args
    match state
        .endereco_loja_service
        .listar_por_loja(args.loja_uuid)
        .await
    {
        Ok(enderecos) => {
            let existing = enderecos.into_iter().find(|e| e.uuid == args.uuid);
            match existing {
                Some(existing) => {
                    let logradouro = args.logradouro.unwrap_or(existing.logradouro);
                    let numero = args.numero.unwrap_or(existing.numero);
                    let bairro = args.bairro.unwrap_or(existing.bairro);
                    let cidade = args.cidade.unwrap_or(existing.cidade);
                    let estado = args.estado.unwrap_or(existing.estado);
                    let cep = args.cep.or(existing.cep);
                    let complemento = args.complemento.or(existing.complemento);

                    let updated = EnderecoLoja {
                        uuid: args.uuid,
                        loja_uuid: args.loja_uuid,
                        cep,
                        logradouro,
                        numero,
                        complemento,
                        bairro,
                        cidade,
                        estado,
                        latitude: existing.latitude,
                        longitude: existing.longitude,
                    };

                    match state.endereco_loja_service.atualizar(updated).await {
                        Ok(()) => print_ok("Endereço de loja atualizado"),
                        Err(e) => print_err(&format!("{:?}", e)),
                    }
                }
                None => print_err("Endereço não encontrado nesta loja"),
            }
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_delete_endereco_loja(state: &AppState, args: DeleteEnderecoLojaArgs) {
    match state.endereco_loja_service.deletar(args.uuid).await {
        Ok(()) => print_ok("Endereço de loja deletado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

// ── Endereco de Usuario ──

pub async fn run_create_endereco_usuario(state: &AppState, args: CreateEnderecoUsuarioArgs) {
    match state
        .endereco_usuario_service
        .criar_endereco(
            args.usuario_uuid,
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
            print_ok("Endereço de usuário criado");
            json_print(&e);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
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

pub async fn run_get_endereco_usuario(state: &AppState, args: GetEnderecoUsuarioArgs) {
    match state
        .endereco_usuario_service
        .buscar_endereco(args.uuid, args.usuario_uuid)
        .await
    {
        Ok(Some(e)) => json_print(&e),
        Ok(None) => print_err("Endereço não encontrado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_endereco_usuario(state: &AppState, args: UpdateEnderecoUsuarioArgs) {
    // Fetch existing address first to get current values, then merge with provided args
    match state
        .endereco_usuario_service
        .buscar_endereco(args.uuid, args.usuario_uuid)
        .await
    {
        Ok(Some(existing)) => {
            let logradouro = args.logradouro.unwrap_or(existing.logradouro);
            let numero = args.numero.unwrap_or(existing.numero);
            let bairro = args.bairro.unwrap_or(existing.bairro);
            let cidade = args.cidade.unwrap_or(existing.cidade);
            let estado = args.estado.unwrap_or(existing.estado);
            let cep = args.cep.or(existing.cep);
            let complemento = args.complemento.or(existing.complemento);

            match state
                .endereco_usuario_service
                .atualizar_endereco(
                    args.uuid,
                    args.usuario_uuid,
                    cep,
                    logradouro,
                    numero,
                    complemento,
                    bairro,
                    cidade,
                    estado,
                    existing.latitude,
                    existing.longitude,
                )
                .await
            {
                Ok(e) => {
                    print_ok("Endereço de usuário atualizado");
                    json_print(&e);
                }
                Err(e) => print_err(&format!("{:?}", e)),
            }
        }
        Ok(None) => print_err("Endereço não encontrado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_delete_endereco_usuario(state: &AppState, args: DeleteEnderecoUsuarioArgs) {
    match state
        .endereco_usuario_service
        .deletar_endereco(args.uuid, args.usuario_uuid)
        .await
    {
        Ok(()) => print_ok("Endereço de usuário deletado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}
