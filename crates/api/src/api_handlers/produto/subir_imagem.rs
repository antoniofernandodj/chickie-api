use axum::{
    Extension, Json, extract::{Path, State, Multipart},
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;
use chickie_core::{models::Usuario};
use crate::{api_handlers::{AppState, dto::AppError}};

pub async fn subir_imagem_produto(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
    Extension(_): Extension<Usuario>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {

    // Extract file from multipart form
    let file = multipart.next_field().await
        .map_err(|e| AppError::BadRequest(format!("Failed to read multipart field: {}", e)))?
        .ok_or_else(|| AppError::BadRequest("No file provided".to_string()))?;

    let content_type = file.content_type()
        .unwrap_or("application/octet-stream")
        .to_string();
    let filename = file.file_name()
        .unwrap_or("image.jpg")
        .to_string();
    let data = file.bytes().await
        .map_err(|e| AppError::BadRequest(format!("Failed to read file data: {}", e)))?;

    // Upload via usecase
    let file_url = state.upload_imagem_usecase.executar(
        uuid,
        filename,
        content_type,
        data,
    ).await.map_err(AppError::Internal)?;

    // Update produto with imagem_url
    state.upload_imagem_usecase.atualizar_produto_imagem(uuid, file_url.clone())
        .await.map_err(AppError::Internal)?;

    Ok(Json(serde_json::json!({
        "uuid": uuid,
        "imagem_url": file_url,
        "message": "Imagem enviada com sucesso"
    })))
}
