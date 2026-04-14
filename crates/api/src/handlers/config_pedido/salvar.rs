use axum::{Extension, extract::{Path, State}};
use uuid::Uuid;
use std::sync::Arc;

use crate::handlers::{dto::AppError, AppState};
use chickie_core::{models::{ConfiguracaoDePedidosLoja, TipoCalculoPedido, Usuario}, usecases::AdminUsecase, proto};
use crate::handlers::protobuf::Protobuf;

pub async fn salvar_config_pedido(
    State(state): State<Arc<AppState>>,
    Path(loja_uuid): Path<Uuid>,
    Extension(usuario): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::SalvarConfigPedidoRequest>,
) -> Result<Protobuf<proto::GenericResponse>, AppError> {
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
    Ok(Protobuf(proto::GenericResponse {
        message: "Configuração de pedido salva com sucesso".to_string(),
        success: true,
    }))
}
