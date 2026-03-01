use axum::{
    Extension, Json, extract::State, response::IntoResponse
};


use std::sync::Arc;
use crate::models::Usuario;
use crate::{api::dto::AppError, models::Produto};
use crate::api::AppState;


pub async fn criar_produto(
    State(state): State<Arc<AppState>>,
    Extension(usuario_logado): Extension<Usuario>, // Injetado pelo middleware
    Json(p): Json<Produto>,
) -> Result<impl IntoResponse, AppError> {

    println!("O usuário {} está criando um pedido", usuario_logado.nome);

    let produto = state
        .catalogo_service
        .criar_produto(
            p.nome,
            p.descricao,
            p.preco, 
            p.categoria_uuid,
            p.loja_uuid,
            p.tempo_preparo_min
        )
        .await?; 

    Ok(Json(produto))
}