use std::collections::HashMap;
use std::sync::Arc;

use axum::{Json, extract::{Path, State}};
use serde::Serialize;
use uuid::Uuid;
use utoipa::ToSchema;

use crate::handlers::{AppState, dto::AppError};
use chickie_core::models::Produto;

#[derive(Serialize, ToSchema)]
pub struct LojaComProdutos {
    pub uuid: Uuid,
    pub produtos: Vec<Produto>,
}

#[derive(Serialize, ToSchema)]
pub struct ProdutosPorCategoriaGlobal {
    pub categoria_uuid: Uuid,
    pub lojas: Vec<LojaComProdutos>,
}

pub async fn listar_produtos_por_categoria_global(
    State(state): State<Arc<AppState>>,
    Path(categoria_uuid): Path<Uuid>,
) -> Result<Json<ProdutosPorCategoriaGlobal>, AppError> {
    let produtos = state.catalogo_service
        .listar_produtos_por_categoria_global(categoria_uuid)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let mut mapa: HashMap<Uuid, Vec<Produto>> = HashMap::new();
    for p in produtos {
        mapa.entry(p.loja_uuid).or_default().push(p);
    }

    let mut lojas: Vec<LojaComProdutos> = mapa
        .into_iter()
        .map(|(uuid, produtos)| LojaComProdutos { uuid, produtos })
        .collect();
    lojas.sort_by_key(|l| l.uuid);

    Ok(Json(ProdutosPorCategoriaGlobal { categoria_uuid, lojas }))
}
