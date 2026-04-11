use uuid::Uuid;

use crate::app_state::AppState;
use crate::args::{AddFavoritoArgs, RemoveFavoritoArgs, VerificarFavoritoArgs};
use crate::helpers::{print_ok, print_err, json_print};

pub async fn run_add_favorito(state: &AppState, args: AddFavoritoArgs) {
    match state
        .loja_favorita_service
        .adicionar_favorita(args.usuario_uuid, args.loja_uuid)
        .await
    {
        Ok(f) => {
            print_ok("Loja adicionada aos favoritos");
            json_print(&f);
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_remove_favorito(state: &AppState, args: RemoveFavoritoArgs) {
    match state
        .loja_favorita_service
        .remover_favorita(args.usuario_uuid, args.loja_uuid)
        .await
    {
        Ok(()) => print_ok("Loja removida dos favoritos"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_favoritas(state: &AppState, usuario_uuid: Uuid) {
    match state
        .loja_favorita_service
        .listar_favoritas(usuario_uuid)
        .await
    {
        Ok(favs) => json_print(&favs),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_verificar_favorito(state: &AppState, args: VerificarFavoritoArgs) {
    match state
        .loja_favorita_service
        .eh_favorita(args.usuario_uuid, args.loja_uuid)
        .await
    {
        Ok(eh) => println!("Favorita: {}", eh),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}
