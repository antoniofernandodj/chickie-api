use axum::{Extension, Json, extract::{Path, State}, response::IntoResponse};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    models::Usuario,
    ports::{PedidoRepositoryPort, UsuarioRepositoryPort},
    usecases::{PagamentoUsecase, PagadorInput},
};
use crate::handlers::{AppState, dto::AppError};

#[derive(Deserialize)]
pub struct PagadorPayload {
    pub nome: Option<String>,
    pub cpf: Option<String>,
}

#[derive(Deserialize)]
pub struct CriarPagamentoRequest {
    pub pagador: Option<PagadorPayload>,
}

pub async fn criar_pagamento(
    State(state): State<Arc<AppState>>,
    Path(pedido_uuid): Path<Uuid>,
    usuario_ext: Option<Extension<Usuario>>,
    Json(body): Json<CriarPagamentoRequest>,
) -> Result<impl IntoResponse, AppError> {
    let usuario = usuario_ext.map(|Extension(u)| u);

    // Validar dados do pagador anônimo antes de instanciar o usecase
    let pagador = if usuario.is_none() {
        let p = body.pagador.ok_or_else(|| {
            AppError::BadRequest("Usuário não autenticado — forneça 'pagador' com nome e CPF".to_string())
        })?;
        let nome = p.nome.ok_or_else(|| AppError::BadRequest("Campo 'pagador.nome' é obrigatório".to_string()))?;
        let cpf = p.cpf.ok_or_else(|| AppError::BadRequest("Campo 'pagador.cpf' é obrigatório".to_string()))?;
        Some(PagadorInput { nome, cpf })
    } else {
        None
    };

    let usecase = PagamentoUsecase::new(
        Arc::clone(&state.asaas_service),
        Arc::clone(&state.pedido_repo) as Arc<dyn PedidoRepositoryPort>,
        Arc::clone(&state.usuario_repo) as Arc<dyn UsuarioRepositoryPort>,
    );

    let output = usecase
        .criar_pagamento_pix(pedido_uuid, usuario.as_ref(), pagador)
        .await
        .map_err(AppError::BadRequest)?;

    Ok(Json(output))
}
