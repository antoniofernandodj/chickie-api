use axum::{Extension, extract::{Path, State}, http::StatusCode};
use std::sync::Arc;
use uuid::Uuid;

use crate::handlers::{dto::AppError, AppState, protobuf::Protobuf};
use chickie_core::{models::Usuario, usecases::AdminUsecase, proto};
use rust_decimal::Decimal;
use chrono::NaiveDate;

pub async fn atualizar_funcionario(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::AtualizarFuncionarioRequest>,
) -> Result<StatusCode, AppError> {

    let data_admissao = NaiveDate::parse_from_str(&p.data_admissao, "%Y-%m-%d")
        .map_err(|e| AppError::BadRequest(format!("Data de admissão inválida. Use o formato YYYY-MM-DD: {}", e)))?;

    let uc = AdminUsecase::new(
        state.ingrediente_service.clone(),
        state.horario_funcionamento_service.clone(),
        state.config_pedido_service.clone(),
        state.funcionario_service.clone(),
        state.entregador_service.clone(),
        state.marketing_service.clone(),
        state.endereco_loja_service.clone(),
        usuario,
        loja_uuid,
    );

    let usuario_uuid = Uuid::parse_str(&p.usuario_uuid)
        .map_err(|e| AppError::BadRequest(format!("Invalid usuario_uuid: {}", e)))?;
    let nome = if p.nome.is_empty() { None } else { Some(p.nome.clone()) };
    let email = if p.email.is_empty() { None } else { Some(p.email.clone()) };
    let senha = if p.senha.is_empty() { None } else { Some(p.senha.clone()) };
    let celular = if p.celular.is_empty() { None } else { Some(p.celular.clone()) };
    let cargo = if p.cargo.is_empty() { None } else { Some(p.cargo.clone()) };
    let salario = if p.salario.is_empty() { None } else { Some(p.salario.parse::<Decimal>().unwrap_or_default()) };

    uc.atualizar_funcionario(
        uuid,
        usuario_uuid,
        nome,
        email,
        senha,
        celular,
        cargo,
        salario,
        data_admissao,
    ).await?;

    Ok(StatusCode::NO_CONTENT)
}
