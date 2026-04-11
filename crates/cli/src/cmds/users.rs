use crate::app_state::AppState;
use crate::helpers::{print_err, json_print};

pub async fn run_list_users(state: &AppState) {
    match state.usuario_service.listar().await {
        Ok(users) => json_print(&users),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}
