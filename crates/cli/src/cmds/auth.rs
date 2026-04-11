use crate::app_state::AppState;
use crate::args::{SignupArgs, LoginArgs};
use crate::helpers::{print_ok, print_err, json_print};

pub async fn run_signup(state: &AppState, args: SignupArgs) {
    match state
        .usuario_service
        .registrar(
            args.nome,
            args.username,
            args.senha,
            args.email,
            args.celular,
            args.auth_method,
            Some(args.classe),
        )
        .await
    {
        Ok(u) => {
            print_ok("Usuário criado");
            json_print(&u);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_login(state: &AppState, args: LoginArgs) {
    match state
        .usuario_service
        .autenticar(args.identifier, args.senha)
        .await
    {
        Ok(token) => {
            print_ok("Login realizado");
            println!("{:?}", token);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}
