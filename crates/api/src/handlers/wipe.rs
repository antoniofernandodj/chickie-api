use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use sqlx::PgPool;
use tracing::info;

use crate::handlers::{AppState, OwnerPermission, protobuf::Protobuf};
use chickie_core::proto;

/// ⚠️ **DEVELOPMENT ONLY** — Wipes ALL data from the database.
/// Must be removed before production deployment.
/// Protected by OwnerPermission — only the platform owner can call this.
pub async fn wipe_database(
    State(state): State<Arc<AppState>>,
    _owner: OwnerPermission,
) -> Result<Protobuf<proto::GenericResponse>, StatusCode> {

    // Only allow in development mode
    let mode = std::env::var("MODE").unwrap_or_default();
    if mode != "development" {
        return Err(StatusCode::FORBIDDEN);
    }

    let pool: &PgPool = state.db.as_ref();

    info!("⚠️  WIPE DATABASE: Truncating all tables");

    // Truncate all tables in dependency order.
    // CASCADE handles FK relationships automatically.
    // RESTART IDENTITY resets auto-increment sequences.
    sqlx::query(
        "TRUNCATE TABLE
            schema_migrations,
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
        tracing::error!("Failed to truncate database: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!("✅ WIPE DATABASE: All tables truncated successfully");

    Ok(Protobuf(proto::GenericResponse {
        message: "Database wiped successfully".to_string(),
        success: true,
    }))
}
