use crate::app_state::AppState;
use crate::args::*;
use crate::helpers::{print_ok, print_err, json_print, parse_decimal};

pub async fn run_create_ingrediente(state: &AppState, args: CreateIngredienteArgs) {
    match state
        .ingrediente_service
        .criar(
            args.loja_uuid,
            args.nome,
            args.descricao,
            parse_decimal(args.preco),
        )
        .await
    {
        Ok(i) => {
            print_ok("Ingrediente criado");
            json_print(&i);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_ingredientes(state: &AppState, args: ListIngredientesArgs) {
    match state
        .ingrediente_service
        .listar_por_loja(args.loja_uuid)
        .await
    {
        Ok(ings) => json_print(&ings),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_ingrediente() {
    print_err("Update ingrediente requires buscar_por_uuid on IngredienteRepositoryPort — not yet implemented");
}

pub async fn run_delete_ingrediente(state: &AppState, args: DeleteIngredienteArgs) {
    match state
        .ingrediente_service
        .deletar(args.uuid)
        .await
    {
        Ok(()) => print_ok("Ingrediente deletado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}
