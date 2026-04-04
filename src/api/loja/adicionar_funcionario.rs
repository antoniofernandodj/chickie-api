use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::{dto::AppError, AppState},
    models::Usuario
};

#[derive(Deserialize)]
pub struct AdicionarFuncionarioRequest {
    pub nome: String,
    pub username: String,
    pub email: String,
    pub senha: String,
    pub celular: String,
    pub cargo: Option<String>,
    pub salario: Option<f64>,
    pub data_admissao: String,
}

pub async fn adicionar_funcionario(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<AdicionarFuncionarioRequest>,
) -> Result<impl IntoResponse, AppError> {

    if !usuario.is_administrador() {
        return Err(AppError::Unauthorized(
            "Apenas administradores podem adicionar funcionários".to_string()
        ));
    }

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
