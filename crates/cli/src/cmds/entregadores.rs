use crate::app_state::AppState;
use crate::args::{
    ListEntregadoresArgs, DeleteEntregadorArgs, ListEntregadoresDisponiveisArgs,
    TrocarEmailSenhaEntregadorArgs, ToggleDisponibilidadeEntregadorArgs, UpdateEntregadorArgs,
};
use crate::helpers::{print_ok, print_err, json_print};

pub async fn run_list_entregadores(state: &AppState, args: ListEntregadoresArgs) {
    match state.entregador_service.listar_por_loja(args.loja_uuid).await {
        Ok(ents) => json_print(&ents),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_entregador(state: &AppState, args: UpdateEntregadorArgs) {
    // Fetch existing entregador to get usuario_uuid and current data
    let ents = match state.entregador_service.listar_por_loja(args.loja_uuid).await {
        Ok(e) => e,
        Err(e) => {
            print_err(&format!("{:?}", e));
            return;
        }
    };
    let existing = ents.into_iter().find(|e| e.uuid == args.uuid);
    let existing = match existing {
        Some(e) => e,
        None => {
            print_err("Entregador não encontrado nesta loja");
            return;
        }
    };

    match state
        .entregador_service
        .atualizar(
            args.uuid,
            existing.usuario_uuid,
            None,  // nome
            None,  // celular
            args.veiculo,
            args.placa,
        )
        .await
    {
        Ok(()) => print_ok("Entregador atualizado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_delete_entregador(state: &AppState, args: DeleteEntregadorArgs) {
    match state.entregador_service.deletar(args.uuid).await {
        Ok(()) => print_ok("Entregador deletado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_list_entregadores_disponiveis(
    state: &AppState,
    args: ListEntregadoresDisponiveisArgs,
) {
    match state.entregador_service.listar_disponiveis(args.loja_uuid).await {
        Ok(ents) => json_print(&ents),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_trocar_email_senha_entregador(
    state: &AppState,
    args: TrocarEmailSenhaEntregadorArgs,
) {
    match state
        .entregador_service
        .trocar_email_senha(args.usuario_uuid, args.novo_email, args.nova_senha)
        .await
    {
        Ok(()) => print_ok("Email/senha do entregador atualizados"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_toggle_disponibilidade_entregador(
    state: &AppState,
    args: ToggleDisponibilidadeEntregadorArgs,
) {
    match state
        .entregador_service
        .definir_disponivel(args.uuid, args.disponivel)
        .await
    {
        Ok(()) => {
            let status = if args.disponivel { "disponível" } else { "indisponível" };
            print_ok(&format!("Entregador marcado como {}", status));
        }
        Err(e) => print_err(&format!("{:?}", e)),
    }
}
