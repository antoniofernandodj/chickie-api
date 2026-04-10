use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;
use rust_decimal::Decimal;

use crate::{api_handlers::{dto::AppError, AppState}, models::{EnderecoLoja, Usuario}, usecases::AdminUsecase};

#[derive(Deserialize)]
pub struct CriarEnderecoLojaRequest {
    pub cep: Option<String>,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
}

pub async fn criar_endereco_loja(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<CriarEnderecoLojaRequest>,
) -> Result<impl IntoResponse, AppError> {
    let endereco = EnderecoLoja::new(
        loja_uuid,
        p.cep,
        p.logradouro,
        p.numero,
        p.complemento,
        p.bairro,
        p.cidade,
        p.estado,
        p.latitude,
        p.longitude,
    );

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
    let uuid = uc.criar_endereco(&endereco).await?;
    Ok((StatusCode::CREATED, Json(serde_json::json!({ "uuid": uuid }))))
}
