use chickie_core::models::HorarioFuncionamento;

use crate::app_state::AppState;
use crate::args::*;
use crate::helpers::{print_ok, print_err, json_print};

pub async fn run_list_horarios(state: &AppState, args: ListHorariosArgs) {
    match state
        .horario_funcionamento_service
        .listar_por_loja(args.loja_uuid)
        .await
    {
        Ok(horarios) => json_print(&horarios),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_create_horario(state: &AppState, args: CreateHorarioArgs) {
    let horario = HorarioFuncionamento::new(
        args.loja_uuid,
        args.dia_semana,
        args.abertura,
        args.fechamento,
    );
    match horario {
        Ok(h) => {
            match state
                .horario_funcionamento_service
                .criar_ou_atualizar(&h)
                .await
            {
                Ok(()) => {
                    print_ok("Horário criado/atualizado");
                }
                Err(e) => print_err(&format!("{:?}", e)),
            }
        }
        Err(e) => print_err(&format!("Horário inválido: {}", e)),
    }
}

pub async fn run_toggle_horario_ativo(state: &AppState, args: ToggleHorarioAtivoArgs) {
    match state
        .horario_funcionamento_service
        .definir_ativo(args.loja_uuid, args.dia_semana, args.ativo)
        .await
    {
        Ok(()) => print_ok("Horário atualizado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}

pub async fn run_delete_horario(state: &AppState, args: DeleteHorarioArgs) {
    match state
        .horario_funcionamento_service
        .deletar_por_dia(args.loja_uuid, args.dia_semana)
        .await
    {
        Ok(()) => print_ok("Horário deletado"),
        Err(e) => print_err(&format!("{:?}", e)),
    }
}
