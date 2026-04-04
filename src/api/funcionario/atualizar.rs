use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;
use rust_decimal::Decimal;

use crate::{api::{dto::AppError, AppState}, models::Usuario, usecases::AdminUsecase};

#[derive(Deserialize)]
pub struct AtualizarFuncionarioRequest {
    pub usuario_uuid: Uuid,
    pub nome: Option<String>,
    pub email: Option<String>,
    pub senha: Option<String>,
    pub celular: Option<String>,
    pub telefone: Option<String>,
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

    let uc = AdminUsecase::new(
        state.ingrediente_service.clone(),
        state.horario_funcionamento_service.clone(),
        state.config_pedido_service.clone(),
        state.funcionario_service.clone(),
        state.entregador_service.clone(),
        state.marketing_service.clone(),
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
        p.telefone,
        p.cargo,
        p.salario,
        p.data_admissao,
    ).await?;
    Ok(StatusCode::NO_CONTENT)
}
