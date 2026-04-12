use chickie_core::ports::PromocaoRepositoryPort;

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

pub async fn run_list_cupons(state: &AppState, _args: ListCuponsArgs) {
    match state
        .marketing_service
        .listar_todos_cupons()
        .await
    {
        Ok(cupons) => json_print(&cupons),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_get_cupom(state: &AppState, args: GetCupomArgs) {
    match state
        .marketing_service
        .buscar_cupom(args.uuid)
        .await
    {
        Ok(c) => json_print(&c),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_validar_cupom(state: &AppState, args: ValidarCupomArgs) {
    match state
        .cupom_repo
        .buscar_por_codigo(&args.codigo, args.loja_uuid)
        .await
    {
        Ok(Some(c)) => json_print(&c),
        Ok(None) => print_err("Cupom não encontrado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_cupom(state: &AppState, args: UpdateCupomArgs) {
    // Buscar cupom existente
    let cupom_existente = match state.marketing_service.buscar_cupom(args.uuid).await {
        Ok(c) => c,
        Err(e) => {
            print_err(&format!("{:?}", e));
            return;
        }
    };

    // Usar valores existentes se não fornecidos
    let codigo = args.codigo.unwrap_or(cupom_existente.codigo);
    let descricao = args.descricao.unwrap_or(cupom_existente.descricao);
    let tipo_desconto = args.tipo_desconto.unwrap_or(cupom_existente.tipo_desconto);
    let valor_desconto = args.valor_desconto.map(|v| parse_decimal(v));
    let valor_minimo = args.valor_minimo.map(|v| parse_decimal(v));
    let data_validade = args.data_validade.unwrap_or_else(|| cupom_existente.data_validade.to_rfc3339());
    let limite_uso = args.limite_uso.or(cupom_existente.limite_uso);

    match state
        .marketing_service
        .atualizar_cupom(
            args.uuid,
            args.loja_uuid,
            codigo,
            descricao,
            tipo_desconto,
            valor_desconto,
            valor_minimo,
            data_validade,
            limite_uso,
        )
        .await
    {
        Ok(()) => print_ok("Cupom atualizado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
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
    
    // Parse datetime strings properly
    let data_inicio = args.data_inicio.clone();
    let data_fim = args.data_fim.clone();
    
    match state
        .marketing_service
        .criar_promocao(
            args.loja_uuid,
            args.nome,
            args.descricao,
            args.tipo_desconto,
            Some(parse_decimal(args.valor_desconto)),
            args.valor_minimo.map(parse_decimal),
            data_inicio,
            data_fim,
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

pub async fn run_update_promocao(state: &AppState, args: UpdatePromocaoArgs) {
    // Fetch existing promotion first to get current values, then merge with provided args
    match state.promocao_repo.buscar_por_uuid(args.uuid).await {
        Ok(Some(existing)) => {
            let nome = args.nome.unwrap_or(existing.nome);
            let descricao = args.descricao.unwrap_or(existing.descricao);
            let tipo_desconto = args.tipo_desconto.unwrap_or(existing.tipo_desconto);
            let valor_desconto = args.valor_desconto.map(|v| parse_decimal(v));
            let valor_minimo = args.valor_minimo.map(|v| parse_decimal(v));
            let data_inicio = args.data_inicio.unwrap_or_else(|| existing.data_inicio.to_rfc3339());
            let data_fim = args.data_fim.unwrap_or_else(|| existing.data_fim.to_rfc3339());
            let dias_semana: Option<Vec<u8>> = args.dias_semana.as_ref().map(|d| d.iter().map(|&x| x as u8).collect());
            let tipo_escopo = args.tipo_escopo.unwrap_or(existing.tipo_escopo);
            let produto_uuid = args.produto_uuid.or(existing.produto_uuid);
            let categoria_uuid = args.categoria_uuid.or(existing.categoria_uuid);
            let prioridade = args.prioridade.unwrap_or(existing.prioridade);

            match state
                .marketing_service
                .atualizar_promocao(
                    args.uuid,
                    args.loja_uuid,
                    nome,
                    descricao,
                    tipo_desconto,
                    valor_desconto,
                    valor_minimo,
                    data_inicio,
                    data_fim,
                    dias_semana,
                    tipo_escopo,
                    produto_uuid,
                    categoria_uuid,
                    prioridade,
                )
                .await
            {
                Ok(()) => print_ok("Promoção atualizada"),
                Err(e) => print_err(&format!("{:?}", e)),
            }
        }
        Ok(None) => print_err("Promoção não encontrada"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
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

// ── Avaliacoes CRUD ──

pub async fn run_list_avaliacoes_loja(state: &AppState, args: ListAvaliacoesLojaArgs) {
    match state
        .marketing_service
        .listar_avaliacoes_loja(args.loja_uuid)
        .await
    {
        Ok(avaliacoes) => json_print(&avaliacoes),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_get_avaliacao_loja(state: &AppState, args: GetAvaliacaoLojaArgs) {
    match state
        .marketing_service
        .buscar_avaliacao_loja_por_uuid(args.uuid)
        .await
    {
        Ok(a) => json_print(&a),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_avaliacao_loja(state: &AppState, args: UpdateAvaliacaoLojaArgs) {
    match state
        .marketing_service
        .atualizar_avaliacao_loja(args.uuid, parse_decimal(args.nota), args.comentario)
        .await
    {
        Ok(a) => {
            print_ok("Avaliação de loja atualizada");
            json_print(&a);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_delete_avaliacao_loja(state: &AppState, args: DeleteAvaliacaoLojaArgs) {
    match state
        .marketing_service
        .deletar_avaliacao_loja(args.uuid)
        .await
    {
        Ok(()) => print_ok("Avaliação de loja deletada"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_avaliacoes_produto(state: &AppState, args: ListAvaliacoesProdutoArgs) {
    match state
        .marketing_service
        .listar_avaliacoes_produto_por_loja(args.loja_uuid)
        .await
    {
        Ok(avaliacoes) => json_print(&avaliacoes),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_avaliacoes_produto_por_produto(state: &AppState, args: ListAvaliacoesProdutoPorProdutoArgs) {
    match state
        .marketing_service
        .listar_avaliacoes_produto_por_produto(args.produto_uuid)
        .await
    {
        Ok(avaliacoes) => json_print(&avaliacoes),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_get_avaliacao_produto(state: &AppState, args: GetAvaliacaoProdutoArgs) {
    match state
        .marketing_service
        .buscar_avaliacao_produto_por_uuid(args.uuid)
        .await
    {
        Ok(a) => json_print(&a),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_avaliacao_produto(state: &AppState, args: UpdateAvaliacaoProdutoArgs) {
    match state
        .marketing_service
        .atualizar_avaliacao_produto(args.uuid, parse_decimal(args.nota), args.descricao, args.comentario)
        .await
    {
        Ok(a) => {
            print_ok("Avaliação de produto atualizada");
            json_print(&a);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_delete_avaliacao_produto(state: &AppState, args: DeleteAvaliacaoProdutoArgs) {
    match state
        .marketing_service
        .deletar_avaliacao_produto(args.uuid)
        .await
    {
        Ok(()) => print_ok("Avaliação de produto deletada"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}
