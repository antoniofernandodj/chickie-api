use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use serde_json::json;
use sqlx::PgPool;
use tracing::info;

use crate::api::AppState;

/// ⚠️ **DEVELOPMENT ONLY** — Wipes ALL data from the database.
/// Must be removed before production deployment.
pub async fn wipe_database(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let pool: &PgPool = state.db.as_ref();

    info!("⚠️  WIPE DATABASE: Truncating all tables");

    // Truncate all tables in dependency order.
    // CASCADE handles FK relationships automatically.
    // RESTART IDENTITY resets auto-increment sequences.
    sqlx::query(
        "TRUNCATE TABLE
            usuarios,
            lojas,
            clientes,
            categorias_produtos,
            produtos,
            adicionais,
            ingredientes,
            enderecos_loja,
            enderecos_usuario,
            enderecos_entrega,
            entregadores,
            funcionarios,
            horarios_funcionamento,
            configuracoes_pedidos_loja,
            pedidos,
            itens_pedido,
            partes_item_pedido,
            adicionais_item_pedido,
            avaliacoes_loja,
            avaliacoes_produto,
            cupons,
            uso_cupons,
            promocoes
        RESTART IDENTITY CASCADE"
    )
    .execute(pool)
    .await
    .map_err(|e| {
        let msg = format!("Failed to truncate database: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": msg })))
    })?;

    info!("✅ WIPE DATABASE: All tables truncated successfully");

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Database wiped successfully",
            "warning": "⚠️ All data has been permanently deleted"
        }))
    ))
}
