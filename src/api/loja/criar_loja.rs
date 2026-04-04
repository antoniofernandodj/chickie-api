use axum::{
    extract::{State, Extension},
    response::{IntoResponse},
    Json
};


use std::sync::Arc;
use crate::{api::{CreateLojaRequest, dto::AppError}, models::{TipoCalculoPedido, Usuario}};
use crate::api::AppState;


pub async fn criar_loja(
    State(state): State<Arc<AppState>>,
    Extension(usuario): Extension<Usuario>,
    Json(p): Json<CreateLojaRequest>,
) -> Result<impl IntoResponse, AppError> {

    // Apenas administradores podem criar lojas
    if !usuario.is_administrador() {
        return Err(AppError::Unauthorized(
            "Apenas administradores podem criar lojas".to_string()
        ));
    }

    let loja = state
        .loja_service
        .criar_loja_completa(
            p.nome,
            p.slug,
            p.email_contato,
            p.descricao,
            p.telefone,
            p.hora_abertura,
            p.hora_fechamento,
            p.dias_funcionamento,
            p.tempo_medio,
            p.nota_media,
            p.taxa_entrega_base,
            p.pedido_minimo,
            p.max_partes,
            TipoCalculoPedido::MaisCaro
        )
        .await?;

    Ok(Json(loja))
}