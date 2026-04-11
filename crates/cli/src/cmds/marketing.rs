use crate::app_state::AppState;
use crate::args::*;
use crate::helpers::{print_ok, print_err, json_print, parse_decimal};

// ── Cupons ──

pub async fn run_create_cupom(state: &AppState, args: CreateCupomArgs) {
    match state
        .marketing_service
        .criar_cupom(
            args.loja_uuid,
            args.codigo,
            args.descricao,
            args.tipo_desconto,
            Some(parse_decimal(args.valor_desconto)),
            Some(parse_decimal(args.valor_minimo)),
            args.data_validade,
            Some(args.limite_uso),
        )
        .await
    {
        Ok(c) => {
            print_ok("Cupom criado");
            json_print(&c);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_cupons() {
    print_err("List cupons precisa de loja_uuid");
}

pub async fn run_validar_cupom() {
    // validar_cupom method doesn't exist on the service
    print_err("ValidarCupom not yet implemented on service");
}

pub async fn run_update_cupom() {
    print_err("Update cupom em construção");
}

pub async fn run_delete_cupom(state: &AppState, args: DeleteCupomArgs) {
    match state
        .marketing_service
        .deletar_cupom(args.uuid)
        .await
    {
        Ok(()) => print_ok("Cupom deletado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

// ── Promocoes ──

pub async fn run_create_promocao(state: &AppState, args: CreatePromocaoArgs) {
    let dias_semana: Option<Vec<u8>> = Some(
        args.dias_semana.iter().map(|&d| d as u8).collect()
    );
    match state
        .marketing_service
        .criar_promocao(
            args.loja_uuid,
            args.nome,
            args.descricao,
            args.tipo_desconto,
            Some(parse_decimal(args.valor_desconto)),
            args.valor_minimo.map(parse_decimal),
            args.data_inicio,
            args.data_fim,
            dias_semana,
            args.tipo_escopo,
            args.produto_uuid,
            args.categoria_uuid,
            args.prioridade,
        )
        .await
    {
        Ok(p) => {
            print_ok("Promoção criada");
            json_print(&p);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_promocoes(state: &AppState, args: ListPromocoesArgs) {
    match state
        .marketing_service
        .listar_promocoes(args.loja_uuid)
        .await
    {
        Ok(promos) => json_print(&promos),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_promocao() {
    print_err("Update promoção em construção");
}

pub async fn run_delete_promocao(state: &AppState, args: DeletePromocaoArgs) {
    match state
        .marketing_service
        .deletar_promocao(args.uuid)
        .await
    {
        Ok(()) => print_ok("Promoção deletada"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

// ── Avaliacoes ──

pub async fn run_avaliar_loja(state: &AppState, args: AvaliarLojaArgs) {
    match state
        .marketing_service
        .avaliar_loja(
            args.loja_uuid,
            args.usuario_uuid,
            parse_decimal(args.nota),
            args.comentario,
        )
        .await
    {
        Ok(a) => {
            print_ok("Loja avaliada");
            json_print(&a);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_avaliar_produto(state: &AppState, args: AvaliarProdutoArgs) {
    match state
        .marketing_service
        .avaliar_produto(
            args.usuario_uuid,
            args.loja_uuid,
            args.produto_uuid,
            args.comentario,
            parse_decimal(args.nota),
            args.descricao,
        )
        .await
    {
        Ok(a) => {
            print_ok("Produto avaliado");
            json_print(&a);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}
