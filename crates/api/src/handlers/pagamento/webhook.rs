use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
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
}

#[derive(Deserialize)]
pub struct WebhookPayment {
    #[serde(rename = "externalReference")]
    pub external_reference: Option<String>,
}

pub async fn webhook_asaas(
    State(state): State<Arc<AppState>>,
    Json(body): Json<WebhookPayload>,
) -> impl IntoResponse {
    let confirmar = matches!(
        body.event.as_str(),
        "PAYMENT_CONFIRMED" | "PAYMENT_RECEIVED"
    );

    if !confirmar {
        return StatusCode::OK;
    }

    let pedido_uuid_str = body
        .payment
        .and_then(|p| p.external_reference)
        .unwrap_or_default();

    let pedido_uuid = match Uuid::parse_str(&pedido_uuid_str) {
        Ok(id) => id,
        Err(_) => {
            tracing::warn!("webhook_asaas: externalReference inválido '{}'", pedido_uuid_str);
            return StatusCode::OK;
        }
    };

    let usecase = PagamentoUsecase::new(
        Arc::clone(&state.asaas_service),
        Arc::clone(&state.pedido_repo) as Arc<dyn PedidoRepositoryPort>,
        Arc::clone(&state.usuario_repo) as Arc<dyn UsuarioRepositoryPort>,
    );

    if let Err(e) = usecase.confirmar_pagamento(pedido_uuid).await {
        tracing::error!("webhook_asaas: falha ao marcar pedido={} como pago: {}", pedido_uuid, e);
    } else {
        tracing::info!("webhook_asaas: pedido={} marcado como pago", pedido_uuid);
    }

    StatusCode::OK
}
