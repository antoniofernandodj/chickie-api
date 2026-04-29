use axum::{extract::State, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use chickie_core::{
    ports::{PedidoRepositoryPort, UsuarioRepositoryPort},
    usecases::PagamentoUsecase,
};
use crate::handlers::AppState;

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub event: String,
    pub payment: Option<WebhookPayment>,
    #[serde(rename = "authToken")]
    pub auth_token: Option<String>,
}

#[derive(Deserialize)]
pub struct WebhookPayment {
    #[serde(rename = "externalReference")]
    pub external_reference: Option<String>,
}

pub async fn webhook_asaas(
    State(state): State<Arc<AppState>>,
    raw: String,
) -> impl IntoResponse {
    tracing::info!(payload = %raw, "webhook_asaas: body bruto recebido");

    let body: WebhookPayload = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!(erro = %e, payload = %raw, "webhook_asaas: falha ao parsear payload");
            return StatusCode::BAD_REQUEST;
        }
    };

    tracing::info!(event = %body.event, tem_auth_token = body.auth_token.is_some(), "webhook_asaas: requisição recebida");

    let token_valido = body
        .auth_token
        .as_deref()
        .map(|t| state.asaas_service.verificar_webhook_token(t))
        .unwrap_or(false);

    if !token_valido {
        tracing::warn!(
            event = %body.event,
            auth_token_presente = body.auth_token.is_some(),
            "webhook_asaas: authToken inválido ou ausente — requisição rejeitada"
        );
        return StatusCode::UNAUTHORIZED;
    }

    tracing::debug!(event = %body.event, "webhook_asaas: authToken válido");

    let confirmar = matches!(
        body.event.as_str(),
        "PAYMENT_CONFIRMED" | "PAYMENT_RECEIVED"
    );

    if !confirmar {
        tracing::info!(event = %body.event, "webhook_asaas: evento ignorado — não é confirmação de pagamento");
        return StatusCode::OK;
    }

    let pedido_uuid_str = body
        .payment
        .and_then(|p| p.external_reference)
        .unwrap_or_default();

    tracing::info!(event = %body.event, external_reference = %pedido_uuid_str, "webhook_asaas: evento de pagamento recebido");

    let pedido_uuid = match Uuid::parse_str(&pedido_uuid_str) {
        Ok(id) => id,
        Err(e) => {
            tracing::warn!(
                external_reference = %pedido_uuid_str,
                erro = %e,
                "webhook_asaas: externalReference inválido — não é um UUID válido"
            );
            return StatusCode::OK;
        }
    };

    tracing::debug!(pedido_uuid = %pedido_uuid, "webhook_asaas: chamando usecase confirmar_pagamento");

    let usecase = PagamentoUsecase::new(
        Arc::clone(&state.asaas_service),
        Arc::clone(&state.pedido_repo) as Arc<dyn PedidoRepositoryPort>,
        Arc::clone(&state.usuario_repo) as Arc<dyn UsuarioRepositoryPort>,
    );

    if let Err(e) = usecase.confirmar_pagamento(pedido_uuid).await {
        tracing::error!(pedido_uuid = %pedido_uuid, event = %body.event, erro = %e, "webhook_asaas: falha ao confirmar pagamento");
    } else {
        tracing::info!(pedido_uuid = %pedido_uuid, event = %body.event, "webhook_asaas: pagamento confirmado com sucesso");
    }

    StatusCode::OK
}
