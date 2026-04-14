use axum::extract::Path;
use axum::{Extension, extract::State};
use uuid::Uuid;
use std::sync::Arc;
use rust_decimal::Decimal;
use chickie_core::usecases::{AtualizarProdutoRequest, CatalogoUsecase};
use chickie_core::ports::to_proto::ToProto;
use chickie_core::{models::Usuario, proto};
use crate::handlers::dto::AppError;
use crate::handlers::{AppState, protobuf::Protobuf};

pub async fn atualizar_produto(
    State(state): State<Arc<AppState>>,
    Extension(usuario_logado): Extension<Usuario>,
    Path(uuid): Path<Uuid>,
    Protobuf(p): Protobuf<proto::AtualizarProdutoRequest>,
) -> Result<Protobuf<proto::Produto>, AppError> {
    let categoria_uuid = Uuid::parse_str(&p.categoria_uuid)
        .map_err(|e| AppError::BadRequest(format!("categoria_uuid inválido: {}", e)))?;
    let request = AtualizarProdutoRequest {
        nome: p.nome,
        descricao: if p.descricao.is_empty() { None } else { Some(p.descricao) },
        preco: Decimal::from_str_exact(&p.preco)
            .map_err(|e| AppError::BadRequest(format!("Preço inválido: {}", e)))?,
        categoria_uuid,
        tempo_preparo_min: if p.tempo_preparo_min == 0 { None } else { Some(p.tempo_preparo_min) },
    };

    let usecase = CatalogoUsecase::new(
        state.catalogo_service.clone(),
        uuid,
        usuario_logado,
    );

    let produto = usecase
        .atualizar_produto(uuid, request)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Protobuf(produto.to_proto()))
}
