use axum::{
    extract::State,
    response::{IntoResponse},
    Json
};

use std::sync::Arc;
use crate::handlers::{CreateLojaRequest, auth::AdminPermission, dto::AppError};
use chickie_core::{models::{TipoCalculoPedido}};
use crate::handlers::AppState;


pub async fn criar_loja(
    State(state): State<Arc<AppState>>,
    AdminPermission(usuario): AdminPermission,
    Json(p): Json<CreateLojaRequest>,
) -> Result<impl IntoResponse, AppError> {

    tracing::info!("usuario {:?} criando loja: {:?}", usuario, p);

    let loja = state
        .loja_service
        .criar_loja_completa(
            p.nome,
            p.slug,
            p.email_contato,
            p.descricao,
            p.celular,
            p.hora_abertura,
            p.hora_fechamento,
            p.dias_funcionamento,
            p.tempo_medio,
            p.nota_media,
            p.taxa_entrega_base,
            p.pedido_minimo,
            usuario.uuid,  // criado_por
            p.max_partes,
            TipoCalculoPedido::MaisCaro
        )
        .await?;

    Ok(Json(loja))
}