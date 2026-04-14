use axum::extract::{Path, State};
use std::sync::Arc;
use uuid::Uuid;
use chrono::NaiveDate;

use rust_decimal::Decimal;
use chickie_core::ports::to_proto::ToProto;
use chickie_core::proto;
use crate::handlers::{AppState, auth::AdminPermission, dto::AppError, protobuf::Protobuf};

pub async fn adicionar_funcionario(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    AdminPermission(_): AdminPermission,
    Protobuf(p): Protobuf<proto::AdicionarFuncionarioRequest>,
) -> Result<Protobuf<proto::Funcionario>, AppError> {

    let data_admissao = NaiveDate::parse_from_str(&p.data_admissao, "%Y-%m-%d")
        .map_err(|e| AppError::BadRequest(format!("Data de admissão inválida. Use o formato YYYY-MM-DD: {}", e)))?;

    let funcionario = state.loja_service.adicionar_funcionario(
        loja_uuid,
        p.nome,
        p.username,
        p.email,
        p.senha,
        p.celular,
        if p.cargo.is_empty() { None } else { Some(p.cargo) },
        if p.salario.is_empty() {
            None
        } else {
            Some(Decimal::from_str_exact(&p.salario)
                .map_err(|e| AppError::BadRequest(format!("Salário inválido: {}", e)))?)
        },
        data_admissao
    ).await?;

    Ok(Protobuf(funcionario.to_proto()))
}
