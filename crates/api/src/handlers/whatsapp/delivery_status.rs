use axum::{
    extract::{State, Form},
    response::IntoResponse,
};
use std::sync::Arc;
use serde::Deserialize;
use crate::handlers::{AppState, dto::AppError};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TwilioDeliveryStatusPayload {
    pub message_sid: String,
    pub message_status: String,
    pub to: String,
}

pub async fn handler(
    State(_state): State<Arc<AppState>>,
    Form(payload): Form<TwilioDeliveryStatusPayload>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("WhatsApp Delivery Status: {:?}", payload);
    Ok(axum::http::StatusCode::OK)
}
