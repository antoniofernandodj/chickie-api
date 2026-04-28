use serde::Deserialize;
use sqlx::PgPool;

use super::ler_seed_json;

#[derive(Deserialize)]
struct CategoriaSeed {
    nome: String,
    descricao: String,
    pizza_mode: bool,
    drink_mode: bool,
}

/// Seeds global categories (`loja_uuid IS NULL`) from `data/seed/categorias_globais.json`.
pub(in crate::database) async fn seed_categorias_globais(pool: &PgPool) -> Result<(), String> {
    let json = ler_seed_json("categorias_globais.json")?;
    let categorias: Vec<CategoriaSeed> = serde_json::from_str(&json)
        .map_err(|e| format!("JSON inválido em categorias_globais.json: {}", e))?;

    for c in categorias {
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM categorias_produtos WHERE loja_uuid IS NULL AND nome = $1)",
        )
        .bind(&c.nome)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Falha ao verificar categoria '{}': {}", c.nome, e))?;

        if !exists {
            sqlx::query(
                "INSERT INTO categorias_produtos (nome, descricao, pizza_mode, drink_mode, loja_uuid)
                 VALUES ($1, $2, $3, $4, NULL)",
            )
            .bind(&c.nome)
            .bind(&c.descricao)
            .bind(c.pizza_mode)
            .bind(c.drink_mode)
            .execute(pool)
            .await
            .map_err(|e| format!("Falha ao semear categoria '{}': {}", c.nome, e))?;

            tracing::info!("   ✅ Categoria global criada: {}", c.nome);
        } else {
            tracing::debug!("   ⏭️  Categoria global já existe: {}", c.nome);
        }
    }

    Ok(())
}
