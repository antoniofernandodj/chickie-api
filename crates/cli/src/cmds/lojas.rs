use chickie_core::models::TipoCalculoPedido;

use crate::app_state::AppState;
use crate::args::*;
use crate::helpers::{print_ok, print_err, json_print, parse_decimal};

pub async fn run_create_loja(state: &AppState, args: CreateLojaArgs) {
    match state
        .loja_service
        .criar_loja_completa(
            args.nome,
            args.slug,
            args.email_contato,
            args.descricao,
            args.celular,
            args.hora_abertura,
            args.hora_fechamento,
            args.dias_funcionamento,
            args.tempo_medio,
            args.nota_media,
            args.taxa_entrega_base,
            args.pedido_minimo,
            args.criado_por,
            args.max_partes,
            TipoCalculoPedido::MaisCaro,
        )
        .await
    {
        Ok(loja) => {
            print_ok("Loja criada");
            json_print(&loja);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_lojas(state: &AppState) {
    match state.loja_service.listar().await {
        Ok(lojas) => json_print(&lojas),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_search_lojas(state: &AppState, args: SearchLojasArgs) {
    match state.loja_service.pesquisar(&args.termo).await {
        Ok(lojas) => json_print(&lojas),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_get_loja_by_slug(state: &AppState, args: GetLojaBySlugArgs) {
    match state.loja_service.buscar_por_slug(&args.slug).await {
        Ok(loja) => json_print(&loja),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_get_loja_by_uuid(state: &AppState, args: GetLojaByUuidArgs) {
    match state.loja_service.buscar_por_uuid(args.uuid).await {
        Ok(loja) => json_print(&loja),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_add_funcionario(state: &AppState, args: AddFuncionarioArgs) {
    let data_admissao = args.data_admissao
        .as_deref()
        .map(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d"))
        .transpose()
        .map_err(|e| format!("Data inválida: {}", e));

    match data_admissao {
        Ok(data) => {
            let data = data.unwrap_or_else(|| chrono::Utc::now().date_naive());
            let salario = if args.salario > 0.0 {
                Some(parse_decimal(args.salario))
            } else {
                None
            };
            match state
                .loja_service
                .adicionar_funcionario(
                    args.loja_uuid,
                    args.nome,
                    args.username,
                    args.email,
                    args.senha,
                    args.celular,
                    args.cargo,
                    salario,
                    data,
                )
                .await
            {
                Ok(f) => {
                    print_ok("Funcionário adicionado");
                    json_print(&f);
                }
                Err(e) => print_err(&format!("{:?}", e)),
            }
        }
        Err(e) => print_err(&e),
    }
}

pub async fn run_add_entregador(state: &AppState, args: AddEntregadorArgs) {
    match state
        .loja_service
        .adicionar_entregador(
            args.loja_uuid,
            args.nome,
            args.username,
            args.email,
            args.senha,
            args.celular,
            args.veiculo,
            args.placa,
        )
        .await
    {
        Ok(e) => {
            print_ok("Entregador adicionado");
            json_print(&e);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_add_cliente(_state: &AppState, _args: AddClienteArgs) {
    // TODO: loja_service.adicionar_cliente needs full user creation flow
    print_err("Comando em construção — usar API por enquanto");
}

pub async fn run_list_minhas_lojas(state: &AppState, args: ListMinhasLojasArgs) {
    match state.loja_service.listar_por_criador(args.admin_uuid).await {
        Ok(lojas) => json_print(&lojas),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_verificar_slug_disponivel(state: &AppState, args: VerificarSlugDisponivelArgs) {
    match state.loja_service.verificar_slug_disponivel(&args.slug).await {
        Ok(disponivel) => {
            if disponivel {
                print_ok(&format!("Slug '{}' está disponível", args.slug));
            } else {
                print_err(&format!("Slug '{}' já está em uso", args.slug));
            }
            println!("{{\"slug\": \"{}\", \"disponivel\": {}}}", args.slug, disponivel);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}
