use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::{dto::AppError, AppState},
    models::Usuario
};

#[derive(Deserialize)]
pub struct AdicionarEntregadorRequest {
    pub nome: String,
    pub telefone: Option<String>,
    pub veiculo: Option<String>,
    pub placa: Option<String>,
}

pub async fn adicionar_entregador(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<AdicionarEntregadorRequest>,
) -> Result<impl IntoResponse, AppError> {

    if !usuario.is_administrador() {
        return Err(AppError::Unauthorized(
            "Apenas administradores podem adicionar entregadores".to_string()
        ));
    }

    let entregador = state.loja_service.adicionar_entregador(
        p.nome,
        loja_uuid,
        p.telefone,
        p.veiculo,
        p.placa
    ).await?;

    Ok(Json(entregador))
}
