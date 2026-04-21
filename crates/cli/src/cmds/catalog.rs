use crate::app_state::AppState;
use crate::args::*;
use crate::helpers::{print_ok, print_err, json_print, parse_decimal};

// ── Produtos ──

pub async fn run_create_produto(state: &AppState, args: CreateProdutoArgs) {
    match state
        .catalogo_service
        .criar_produto(
            args.nome,
            args.descricao,
            parse_decimal(args.preco),
            args.categoria_uuid,
            args.loja_uuid,
            Some(args.tempo_preparo_min),
        )
        .await
    {
        Ok(p) => {
            print_ok("Produto criado");
            json_print(&p);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_produtos() {
    print_err("ListProdutos precisa de loja_uuid — use ListProdutosPorLoja");
}

pub async fn run_list_produtos_por_loja(state: &AppState, args: ListProdutosPorLojaArgs) {
    match state
        .catalogo_service
        .listar_produtos_de_loja(args.loja_uuid)
        .await
    {
        Ok(prods) => json_print(&prods),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_get_produto(state: &AppState, args: GetProdutoArgs) {
    match state
        .catalogo_service
        .buscar_produto_por_uuid(args.uuid)
        .await
    {
        Ok(p) => json_print(&p),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_produto(state: &AppState, args: UpdateProdutoArgs) {
    match state
        .catalogo_service
        .atualizar_produto(
            args.uuid,
            args.nome,
            args.descricao,
            parse_decimal(args.preco),
            args.categoria_uuid,
            Some(args.tempo_preparo_min),
        )
        .await
    {
        Ok(p) => {
            print_ok("Produto atualizado");
            json_print(&p);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_delete_produto(state: &AppState, args: DeleteProdutoArgs) {
    match state.catalogo_service.deletar_produto(args.uuid).await {
        Ok(()) => print_ok("Produto deletado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

// ── Categorias ──

pub async fn run_create_categoria(state: &AppState, args: CreateCategoriaArgs) {
    match state
        .catalogo_service
        .criar_categoria(args.nome, args.descricao, Some(args.loja_uuid), args.pizza_mode, args.drink_mode)
        .await
    {
        Ok(c) => {
            print_ok("Categoria criada");
            json_print(&c);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_categorias(state: &AppState, args: ListCategoriasArgs) {
    match state
        .catalogo_service
        .listar_categorias(args.loja_uuid)
        .await
    {
        Ok(cats) => json_print(&cats),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_categoria(state: &AppState, args: UpdateCategoriaArgs) {
    match state
        .catalogo_service
        .atualizar_categoria(
            args.uuid,
            args.loja_uuid,
            args.nome,
            args.descricao,
            args.pizza_mode,
            args.drink_mode,
        )
        .await
    {
        Ok(c) => {
            print_ok("Categoria atualizada");
            json_print(&c);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_delete_categoria(state: &AppState, args: DeleteCategoriaArgs) {
    match state
        .catalogo_service
        .deletar_categoria(args.uuid, args.loja_uuid)
        .await
    {
        Ok(()) => print_ok("Categoria deletada"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

// ── Adicionais ──

pub async fn run_create_adicional(state: &AppState, args: CreateAdicionalArgs) {
    match state
        .catalogo_service
        .criar_adicional(args.nome, args.loja_uuid, args.descricao, parse_decimal(args.preco))
        .await
    {
        Ok(a) => {
            print_ok("Adicional criado");
            json_print(&a);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_adicionais(state: &AppState, args: ListAdicionaisArgs) {
    match state
        .catalogo_service
        .listar_adicionais(args.loja_uuid)
        .await
    {
        Ok(adics) => json_print(&adics),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_adicional(state: &AppState, args: UpdateAdicionalArgs) {
    match state
        .catalogo_service
        .atualizar_adicional(args.uuid, args.loja_uuid, args.nome, args.descricao, parse_decimal(args.preco))
        .await
    {
        Ok(a) => {
            print_ok("Adicional atualizado");
            json_print(&a);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_delete_adicional(state: &AppState, args: DeleteAdicionalArgs) {
    match state
        .catalogo_service
        .deletar_adicional(args.uuid, args.loja_uuid)
        .await
    {
        Ok(()) => print_ok("Adicional deletado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

// ── Disponibilidade ──

pub async fn run_list_adicionais_disponiveis(state: &AppState, args: ListAdicionaisDisponiveisArgs) {
    match state
        .catalogo_service
        .listar_adicionais_disponiveis(args.loja_uuid)
        .await
    {
        Ok(adics) => json_print(&adics),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_produtos_por_categoria(state: &AppState, args: ListProdutosPorCategoriaArgs) {
    match state
        .catalogo_service
        .listar_produtos_por_categoria(args.categoria_uuid)
        .await
    {
        Ok(prods) => json_print(&prods),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_toggle_disponibilidade_adicional(
    state: &AppState,
    args: ToggleDisponibilidadeAdicionalArgs,
) {
    match state
        .catalogo_service
        .atualizar_disponibilidade(args.uuid, args.loja_uuid, args.disponivel)
        .await
    {
        Ok(()) => {
            let status = if args.disponivel { "disponível" } else { "indisponível" };
            print_ok(&format!("Adicional marcado como {}", status));
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_toggle_disponibilidade_produto(
    state: &AppState,
    args: ToggleDisponibilidadeProdutoArgs,
) {
    match state
        .catalogo_service
        .atualizar_disponibilidade_produto(args.uuid, args.loja_uuid, args.disponivel)
        .await
    {
        Ok(()) => {
            let status = if args.disponivel { "disponível" } else { "indisponível" };
            print_ok(&format!("Produto marcado como {}", status));
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}
