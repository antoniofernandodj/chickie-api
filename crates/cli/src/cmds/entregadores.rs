use crate::app_state::AppState;
use crate::args::ListEntregadoresArgs;
use crate::helpers::{print_err, json_print};

pub async fn run_list_entregadores(state: &AppState, args: ListEntregadoresArgs) {
    match state.entregador_service.listar_por_loja(args.loja_uuid).await {
        Ok(ents) => json_print(&ents),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_entregador() {
    print_err("Update entregador em construção");
}
