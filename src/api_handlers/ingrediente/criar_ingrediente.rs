use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

use crate::{
    api_handlers::{dto::AppError, AppState},
    models::Usuario,
    usecases::AdminUsecase
};

#[derive(Deserialize)]
pub struct CriarIngredienteRequest {
    pub nome: String,
    pub unidade_medida: Option<String>,
    pub preco_unitario: f64,
}

pub async fn criar_ingrediente(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<CriarIngredienteRequest>,
) -> Result<impl IntoResponse, AppError> {
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
    let ingrediente = uc.criar_ingrediente(p.nome, p.unidade_medida, p.preco_unitario).await?;
    Ok(Json(ingrediente))
}
