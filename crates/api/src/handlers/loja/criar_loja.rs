use axum::{
    extract::State,
};

use std::sync::Arc;
use crate::handlers::{auth::AdminPermission, dto::AppError, protobuf::Protobuf};
use chickie_core::ports::to_proto::ToProto;
use chickie_core::{models::TipoCalculoPedido, proto};
use crate::handlers::AppState;


pub async fn criar_loja(
    State(state): State<Arc<AppState>>,
    AdminPermission(usuario): AdminPermission,
    Protobuf(p): Protobuf<proto::CreateLojaRequest>,
) -> Result<Protobuf<proto::Loja>, AppError> {

    tracing::info!("usuario {:?} criando loja: {:?}", usuario, p);

    let loja = state
        .loja_service
        .criar_loja_completa(
            p.nome,
            p.slug,
            p.email_contato,
            if p.descricao.is_empty() { None } else { Some(p.descricao) },
            if p.celular.is_empty() { None } else { Some(p.celular) },
            if p.hora_abertura.is_empty() { None } else { Some(p.hora_abertura) },
            if p.hora_fechamento.is_empty() { None } else { Some(p.hora_fechamento) },
            if p.dias_funcionamento.is_empty() { None } else { Some(p.dias_funcionamento) },
            if p.tempo_medio == 0 { None } else { Some(p.tempo_medio) },
            if p.nota_media == 0.0 { None } else { Some(p.nota_media) },
            if p.taxa_entrega_base == 0.0 { None } else { Some(p.taxa_entrega_base) },
            if p.pedido_minimo == 0.0 { None } else { Some(p.pedido_minimo) },
            usuario.uuid,  // criado_por
            p.max_partes,
            TipoCalculoPedido::MaisCaro
        )
        .await?;

    Ok(Protobuf(loja.to_proto()))
}
