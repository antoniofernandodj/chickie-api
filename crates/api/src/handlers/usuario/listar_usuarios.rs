use axum::{
    extract::{Query, State},
    Json
};
use serde::Deserialize;
use std::sync::Arc;
use crate::handlers::{AppState, dto::AppError, OwnerPermission};
use chickie_core::{models, repositories::Repository};

#[derive(Deserialize)]
pub struct ListarUsuariosQuery {
    pub classe: Option<String>,
}

pub async fn listar_usuarios(
    State(state): State<Arc<AppState>>,
    _owner: OwnerPermission,
    Query(query): Query<ListarUsuariosQuery>,
) -> Result<Json<Vec<models::Usuario>>, AppError> {

    let usuarios = if let Some(classe) = query.classe {
        state.usuario_repo
            .listar_por_classe(&classe)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
    } else {
        state.usuario_repo
            .listar_todos()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
    };

    // Filtrar usuários owner — nunca devem aparecer na lista
    let owner_email = std::env::var("OWNER_EMAIL").unwrap_or_default();
    let usuarios_filtrados: Vec<_> = usuarios.into_iter()
        .filter(|u| u.classe != "owner")
        .filter(|u| owner_email.is_empty() || u.email != owner_email)
        .collect();

    Ok(Json(usuarios_filtrados))
}