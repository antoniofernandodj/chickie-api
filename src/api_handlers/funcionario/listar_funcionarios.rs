use axum::{Extension, Json, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api_handlers::{AppState, dto::AppError},
    models::Usuario,
    usecases::AdminUsecase,
};

pub async fn listar_funcionarios(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
) -> Result<Json<Vec<crate::models::Funcionario>>, AppError> {

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

    let funcionarios = uc.listar_funcionarios().await?;
    Ok(Json(funcionarios))
}
