use crate::app_state::AppState;
use crate::args::{VerificarEmailDisponivelArgs, VerificarUsernameDisponivelArgs};
use crate::helpers::{print_ok, print_err, json_print};

pub async fn run_list_users(state: &AppState) {
    match state.usuario_service.listar().await {
        Ok(users) => json_print(&users),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_verificar_email_disponivel(state: &AppState, args: VerificarEmailDisponivelArgs) {
    match state.usuario_service.verificar_email_disponivel(&args.email).await {
        Ok(disponivel) => {
            if disponivel {
                print_ok(&format!("Email '{}' está disponível", args.email));
            } else {
                print_err(&format!("Email '{}' já está em uso", args.email));
            }
            println!("{{\"email\": \"{}\", \"disponivel\": {}}}", args.email, disponivel);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_verificar_username_disponivel(state: &AppState, args: VerificarUsernameDisponivelArgs) {
    match state.usuario_service.verificar_username_disponivel(&args.username).await {
        Ok(disponivel) => {
            if disponivel {
                print_ok(&format!("Username '{}' está disponível", args.username));
            } else {
                print_err(&format!("Username '{}' já está em uso", args.username));
            }
            println!("{{\"username\": \"{}\", \"disponivel\": {}}}", args.username, disponivel);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}
