use chrono::NaiveDate;

use crate::app_state::AppState;
use crate::args::{ListFuncionariosArgs, DeleteFuncionarioArgs, TrocarEmailSenhaFuncionarioArgs, UpdateFuncionarioArgs};
use crate::helpers::{print_ok, print_err, json_print};

pub async fn run_list_funcionarios(state: &AppState, args: ListFuncionariosArgs) {
    match state.funcionario_service.listar_por_loja(args.loja_uuid).await {
        Ok(funcs) => json_print(&funcs),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_update_funcionario(state: &AppState, args: UpdateFuncionarioArgs) {
    // Fetch existing funcionario to get usuario_uuid and current data
    let funcs = match state.funcionario_service.listar_por_loja(args.loja_uuid).await {
        Ok(f) => f,
        Err(e) => {
            print_err(&format!("{:?}", e));
            return;
        }
    };
    let existing = funcs.into_iter().find(|f| f.uuid == args.uuid);
    let existing = match existing {
        Some(e) => e,
        None => {
            print_err("Funcionário não encontrado nesta loja");
            return;
        }
    };

    let data_admissao = args.data_admissao
        .as_deref()
        .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d"))
        .transpose();

    match data_admissao {
        Ok(Some(data)) => {
            match state
                .funcionario_service
                .atualizar(
                    args.uuid,
                    existing.usuario_uuid,
                    None,  // nome
                    None,  // email
                    None,  // senha
                    None,  // celular
                    args.cargo,
                    args.salario.map(|s| rust_decimal::Decimal::from_f64_retain(s).unwrap_or(rust_decimal::Decimal::ZERO)),
                    data,
                )
                .await
            {
                Ok(()) => print_ok("Funcionário atualizado"),
                Err(e) => print_err(&format!("{:?}", e)),
            }
        }
        Ok(None) => {
            match state
                .funcionario_service
                .atualizar(
                    args.uuid,
                    existing.usuario_uuid,
                    None,
                    None,
                    None,
                    None,
                    args.cargo,
                    args.salario.map(|s| rust_decimal::Decimal::from_f64_retain(s).unwrap_or(rust_decimal::Decimal::ZERO)),
                    existing.data_admissao,
                )
                .await
            {
                Ok(()) => print_ok("Funcionário atualizado"),
                Err(e) => print_err(&format!("{:?}", e)),
            }
        }
        Err(e) => print_err(&format!("Data inválida: {}", e)),
    }
}

pub async fn run_delete_funcionario(state: &AppState, args: DeleteFuncionarioArgs) {
    match state.funcionario_service.deletar(args.uuid).await {
        Ok(()) => print_ok("Funcionário deletado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_trocar_email_senha_funcionario(
    state: &AppState,
    args: TrocarEmailSenhaFuncionarioArgs,
) {
    match state
        .funcionario_service
        .trocar_email_senha(args.usuario_uuid, args.novo_email, args.nova_senha)
        .await
    {
        Ok(()) => print_ok("Email/senha do funcionário atualizados"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}
