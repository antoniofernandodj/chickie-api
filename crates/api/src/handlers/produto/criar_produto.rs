use axum::{Extension, extract::State};
use std::sync::Arc;
use rust_decimal::Decimal;
use uuid::Uuid;
use chickie_core::usecases::{CatalogoUsecase, CreateProdutoRequest};
use chickie_core::ports::to_proto::ToProto;
use chickie_core::{models::Usuario, proto};
use crate::handlers::dto::AppError;
use crate::handlers::{AppState, protobuf::Protobuf};

pub async fn criar_produto(
    State(state): State<Arc<AppState>>,
    Extension(usuario_logado): Extension<Usuario>,
    Protobuf(p): Protobuf<proto::CreateProdutoRequest>,
) -> Result<Protobuf<proto::Produto>, AppError> {
    let loja_uuid = Uuid::parse_str(&p.loja_uuid)
        .map_err(|e| AppError::BadRequest(format!("loja_uuid inválido: {}", e)))?;
    let categoria_uuid = Uuid::parse_str(&p.categoria_uuid)
        .map_err(|e| AppError::BadRequest(format!("categoria_uuid inválido: {}", e)))?;
    let service = state.catalogo_service.clone();
    let usecase: CatalogoUsecase =
        CatalogoUsecase::new(service, loja_uuid, usuario_logado);
    let request = CreateProdutoRequest {
        uuid: None,
        loja_uuid,
        categoria_uuid,
        nome: p.nome,
        descricao: if p.descricao.is_empty() { None } else { Some(p.descricao) },
        preco: Decimal::from_str_exact(&p.preco)
            .map_err(|e| AppError::BadRequest(format!("Preço inválido: {}", e)))?,
        imagem_url: if p.imagem_url.is_empty() { None } else { Some(p.imagem_url) },
        disponivel: p.disponivel,
        tempo_preparo_min: if p.tempo_preparo_min == 0 { None } else { Some(p.tempo_preparo_min) },
        destaque: p.destaque,
    };
    
    let produto = usecase
        .criar_produto(request)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Protobuf(produto.to_proto()))
}
