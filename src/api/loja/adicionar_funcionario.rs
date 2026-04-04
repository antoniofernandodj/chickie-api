use axum::{Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::{
    api::{AppState, auth::AdminPermission, dto::AppError},
};

#[derive(Deserialize)]
pub struct AdicionarFuncionarioRequest {
    pub nome: String,
    pub username: String,
    pub email: String,
    pub senha: String,
    pub celular: String,
    pub cargo: Option<String>,
    pub salario: Option<Decimal>,
    pub data_admissao: String,
}

pub async fn adicionar_funcionario(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    AdminPermission(_): AdminPermission,
    Json(p): Json<AdicionarFuncionarioRequest>,
) -> Result<impl IntoResponse, AppError> {

    let funcionario = state.loja_service.adicionar_funcionario(
        loja_uuid,
        p.nome,
        p.username,
        p.email,
        p.senha,
        p.celular,
        p.cargo,
        p.salario,
        p.data_admissao
    ).await?;

    Ok(Json(funcionario))
}
