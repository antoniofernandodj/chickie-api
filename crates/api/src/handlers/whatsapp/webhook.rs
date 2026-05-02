use axum::{
    extract::{State, Form},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use serde::Deserialize;
use crate::handlers::{AppState, dto::AppError};
use chickie_core::usecases::WhatsAppUsecase;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TwilioWebhookPayload {
    pub message_sid: String,
    pub from: String,
    pub to: String,
    pub body: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Form(payload): Form<TwilioWebhookPayload>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Recebendo webhook do WhatsApp: From={}, SID={}, Body={}", payload.from, payload.message_sid, payload.body);

    let usecase = WhatsAppUsecase::new(state.whatsapp_service.clone());

    let response_text = usecase
        .receber_webhook(
            &payload.from,
            &payload.message_sid,
            &payload.body
        )
        .await
        .map_err(|e| {
            tracing::error!("Erro ao processar webhook do WhatsApp: {:?}", e);
            e
        })?;

    if response_text.is_empty() {
        tracing::debug!("Resposta do WhatsApp vazia (idempotência ou sem comando)");
        return Ok(axum::http::StatusCode::OK.into_response());
    }

    tracing::info!("Enviando resposta para WhatsApp: {}", response_text);
    
    let twiml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><Response><Message>{}</Message></Response>"#,
        response_text
    );

    Ok((
        [("Content-Type", "application/xml")],
        twiml
    ).into_response())
}
