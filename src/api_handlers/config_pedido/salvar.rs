use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

use crate::{api::{dto::AppError, AppState}, models::{ConfiguracaoDePedidosLoja, TipoCalculoPedido, Usuario}, usecases::AdminUsecase};

#[derive(Deserialize)]
pub struct SalvarConfigPedidoRequest {
    pub max_partes: i32,
    pub tipo_calculo: String,
}

pub async fn salvar_config_pedido(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<SalvarConfigPedidoRequest>,
) -> Result<impl IntoResponse, AppError> {
    let tipo_calculo = TipoCalculoPedido::from_str(&p.tipo_calculo)
        .map_err(|e| AppError::BadRequest(e))?;

    let config = ConfiguracaoDePedidosLoja::new(
        loja_uuid,
        p.max_partes,
        tipo_calculo,
    ).map_err(|e| AppError::BadRequest(e))?;

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
    uc.salvar_config_pedido(&config).await?;
    Ok(StatusCode::NO_CONTENT)
}
