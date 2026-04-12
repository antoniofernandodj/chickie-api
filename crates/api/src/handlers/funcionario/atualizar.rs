use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;
use rust_decimal::Decimal;
use chrono::NaiveDate;

use crate::handlers::{dto::AppError, AppState};
use chickie_core::{models::Usuario, usecases::AdminUsecase};

#[derive(Deserialize)]
pub struct AtualizarFuncionarioRequest {
    pub usuario_uuid: Uuid,
    pub nome: Option<String>,
    pub email: Option<String>,
    pub senha: Option<String>,
    pub celular: Option<String>,
    pub cargo: Option<String>,
    pub salario: Option<Decimal>,
    pub data_admissao: String,
}

pub async fn atualizar_funcionario(
    State(state): State<Arc<AppState>>,
    Path((loja_uuid, uuid)): Path<(Uuid, Uuid)>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<AtualizarFuncionarioRequest>,
) -> Result<impl IntoResponse, AppError> {

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

    uc.atualizar_funcionario(
        uuid,
        p.usuario_uuid,
        p.nome,
        p.email,
        p.senha,
        p.celular,
        p.cargo,
        p.salario,
        data_admissao,
    ).await?;
    Ok(StatusCode::NO_CONTENT)
}
