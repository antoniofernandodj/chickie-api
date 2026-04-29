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

    tracing::info!(
        pedido_uuid = %pedido_uuid,
        autenticado = usuario.is_some(),
        usuario_uuid = usuario.as_ref().map(|u| u.uuid.to_string()).as_deref().unwrap_or("anonimo"),
        "criar_pagamento: requisição recebida"
    );

    let pagador = if usuario.is_none() {
        tracing::debug!(pedido_uuid = %pedido_uuid, "criar_pagamento: fluxo anonimo — validando campos de pagador");

        let p = body.pagador.ok_or_else(|| {
            tracing::warn!(pedido_uuid = %pedido_uuid, "criar_pagamento: campo 'pagador' ausente para usuario anonimo");
            AppError::BadRequest("Usuário não autenticado — forneça 'pagador' com nome e CPF".to_string())
        })?;
        let nome = p.nome.ok_or_else(|| {
            tracing::warn!(pedido_uuid = %pedido_uuid, "criar_pagamento: campo 'pagador.nome' ausente");
            AppError::BadRequest("Campo 'pagador.nome' é obrigatório".to_string())
        })?;
        let cpf = p.cpf.ok_or_else(|| {
            tracing::warn!(pedido_uuid = %pedido_uuid, "criar_pagamento: campo 'pagador.cpf' ausente");
            AppError::BadRequest("Campo 'pagador.cpf' é obrigatório".to_string())
        })?;

        tracing::debug!(pedido_uuid = %pedido_uuid, nome = %nome, "criar_pagamento: pagador anonimo validado");
        Some(PagadorInput { nome, cpf })
    } else {
        tracing::debug!(pedido_uuid = %pedido_uuid, "criar_pagamento: fluxo autenticado — pagador resolvido via usuario");
        None
    };

    let usecase = PagamentoUsecase::new(
        Arc::clone(&state.asaas_service),
        Arc::clone(&state.pedido_repo) as Arc<dyn PedidoRepositoryPort>,
        Arc::clone(&state.usuario_repo) as Arc<dyn UsuarioRepositoryPort>,
    );

    tracing::debug!(pedido_uuid = %pedido_uuid, "criar_pagamento: chamando usecase");

    let output = usecase
        .criar_pagamento_pix(pedido_uuid, usuario.as_ref(), pagador)
        .await
        .map_err(|e| {
            tracing::error!(pedido_uuid = %pedido_uuid, erro = %e, "criar_pagamento: usecase retornou erro");
            AppError::BadRequest(e)
        })?;

    tracing::info!(
        pedido_uuid = %pedido_uuid,
        payment_id = %output.payment_id,
        vencimento = %output.vencimento,
        "criar_pagamento: pagamento PIX criado com sucesso"
    );

    Ok(Json(output))
}
