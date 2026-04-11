use crate::app_state::AppState;
use crate::args::ListFuncionariosArgs;
use crate::helpers::{print_err, json_print};

pub async fn run_list_funcionarios(state: &AppState, args: ListFuncionariosArgs) {
    match state.funcionario_service.listar_por_loja(args.loja_uuid).await {
        Ok(funcs) => json_print(&funcs),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_funcionario() {
    print_err("Update funcionário em construção");
}
