use std::sync::Arc;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    models::Usuario,
    ports::{PedidoRepositoryPort, UsuarioRepositoryPort},
    services::AsaasService,
};

pub struct PagadorInput {
    pub nome: String,
    pub cpf: String,
}

#[derive(Serialize)]
pub struct PagamentoOutput {
    pub payment_id: String,
    pub qr_code_image: String,
    pub pix_copia_cola: String,
    pub vencimento: String,
}

pub struct PagamentoUsecase {
    asaas: Arc<AsaasService>,
    pedido_repo: Arc<dyn PedidoRepositoryPort>,
    usuario_repo: Arc<dyn UsuarioRepositoryPort>,
}

impl PagamentoUsecase {
    pub fn new(
        asaas: Arc<AsaasService>,
        pedido_repo: Arc<dyn PedidoRepositoryPort>,
        usuario_repo: Arc<dyn UsuarioRepositoryPort>,
    ) -> Self {
        Self { asaas, pedido_repo, usuario_repo }
    }

    pub async fn criar_pagamento_pix(
        &self,
        pedido_uuid: Uuid,
        usuario: Option<&Usuario>,
        pagador: Option<PagadorInput>,
    ) -> Result<PagamentoOutput, String> {
        tracing::info!(pedido_uuid = %pedido_uuid, "pagamento_usecase: iniciando criação de pagamento PIX");

        // 1. Buscar pedido
        tracing::debug!(pedido_uuid = %pedido_uuid, "pagamento_usecase: buscando pedido no banco");
        let pedido = self.pedido_repo
            .buscar_por_uuid(pedido_uuid)
            .await
            .map_err(|e| {
                tracing::error!(pedido_uuid = %pedido_uuid, erro = %e, "pagamento_usecase: erro ao buscar pedido");
                e.to_string()
            })?
            .ok_or_else(|| {
                tracing::warn!(pedido_uuid = %pedido_uuid, "pagamento_usecase: pedido não encontrado");
                format!("Pedido {} não encontrado", pedido_uuid)
            })?;

        tracing::debug!(
            pedido_uuid = %pedido_uuid,
            total = %pedido.total,
            pago = pedido.pago,
            "pagamento_usecase: pedido encontrado"
        );

        if pedido.pago {
            tracing::warn!(pedido_uuid = %pedido_uuid, "pagamento_usecase: pedido já foi pago — abortando");
            return Err("Pedido já foi pago".to_string());
        }

        // 2. Resolver nome, cpf e asaas_customer_id_cache
        tracing::debug!(
            pedido_uuid = %pedido_uuid,
            autenticado = usuario.is_some(),
            "pagamento_usecase: resolvendo dados do pagador"
        );

        let (nome, cpf, asaas_id_cache, usuario_uuid) = match usuario {
            Some(u) => {
                tracing::debug!(
                    pedido_uuid = %pedido_uuid,
                    usuario_uuid = %u.uuid,
                    tem_asaas_id_cache = u.asaas_customer_id.is_some(),
                    "pagamento_usecase: pagador autenticado resolvido"
                );
                (
                    u.nome.clone(),
                    u.cpf.clone(),
                    u.asaas_customer_id.clone(),
                    Some(u.uuid),
                )
            }
            None => {
                let p = pagador.ok_or_else(|| {
                    tracing::error!(pedido_uuid = %pedido_uuid, "pagamento_usecase: pagador anonimo sem dados");
                    "Usuário não autenticado — forneça nome e CPF no campo 'pagador'".to_string()
                })?;
                let cpf_limpo: String = p.cpf.chars().filter(|c| c.is_ascii_digit()).collect();
                tracing::debug!(
                    pedido_uuid = %pedido_uuid,
                    nome = %p.nome,
                    cpf_len = cpf_limpo.len(),
                    "pagamento_usecase: pagador anonimo resolvido"
                );
                (p.nome, cpf_limpo, None, None)
            }
        };

        // 3. Obter ou criar customer no Asaas
        let asaas_customer_id = match asaas_id_cache {
            Some(ref id) => {
                tracing::info!(
                    pedido_uuid = %pedido_uuid,
                    asaas_customer_id = %id,
                    "pagamento_usecase: customer Asaas resolvido via cache do usuario"
                );
                id.clone()
            }
            None => {
                tracing::info!(
                    pedido_uuid = %pedido_uuid,
                    nome = %nome,
                    "pagamento_usecase: customer Asaas não cacheado — buscando/criando no Asaas"
                );
                let id = self.asaas
                    .cadastrar_ou_buscar_usuario_no_asaas(&nome, &cpf)
                    .await
                    .map_err(|e| {
                        tracing::error!(
                            pedido_uuid = %pedido_uuid,
                            nome = %nome,
                            erro = %e,
                            "pagamento_usecase: falha ao obter customer no Asaas"
                        );
                        e
                    })?;

                tracing::info!(
                    pedido_uuid = %pedido_uuid,
                    asaas_customer_id = %id,
                    "pagamento_usecase: customer Asaas obtido"
                );

                if let Some(uid) = usuario_uuid {
                    tracing::debug!(usuario_uuid = %uid, asaas_customer_id = %id, "pagamento_usecase: persistindo asaas_customer_id no usuario");
                    if let Err(e) = self.usuario_repo
                        .salvar_asaas_customer_id(uid, &id)
                        .await
                    {
                        tracing::warn!(
                            usuario_uuid = %uid,
                            asaas_customer_id = %id,
                            erro = %e,
                            "pagamento_usecase: falha ao salvar asaas_customer_id — continuando sem persistir"
                        );
                    } else {
                        tracing::debug!(usuario_uuid = %uid, "pagamento_usecase: asaas_customer_id persistido com sucesso");
                    }
                }
                id
            }
        };

        // 4. Criar cobrança PIX
        tracing::info!(
            pedido_uuid = %pedido_uuid,
            asaas_customer_id = %asaas_customer_id,
            total = %pedido.total,
            "pagamento_usecase: criando cobrança PIX no Asaas"
        );

        let pagamento = self.asaas
            .criar_cobranca_pix(&asaas_customer_id, pedido.total, pedido_uuid)
            .await
            .map_err(|e| {
                tracing::error!(
                    pedido_uuid = %pedido_uuid,
                    asaas_customer_id = %asaas_customer_id,
                    erro = %e,
                    "pagamento_usecase: falha ao criar cobrança PIX"
                );
                e
            })?;

        tracing::info!(
            pedido_uuid = %pedido_uuid,
            payment_id = %pagamento.payment_id,
            vencimento = %pagamento.vencimento,
            "pagamento_usecase: cobrança PIX criada com sucesso"
        );

        Ok(PagamentoOutput {
            payment_id: pagamento.payment_id,
            qr_code_image: pagamento.qr_code_image,
            pix_copia_cola: pagamento.pix_copia_cola,
            vencimento: pagamento.vencimento,
        })
    }

    /// Chamado pelo webhook do Asaas quando o pagamento é confirmado.
    pub async fn confirmar_pagamento(&self, pedido_uuid: Uuid) -> Result<(), String> {
        tracing::info!(pedido_uuid = %pedido_uuid, "pagamento_usecase: confirmando pagamento via webhook");

        self.pedido_repo
            .marcar_como_pago(pedido_uuid)
            .await
            .map_err(|e| {
                tracing::error!(pedido_uuid = %pedido_uuid, erro = %e, "pagamento_usecase: falha ao marcar pedido como pago");
                e.to_string()
            })?;

        tracing::info!(pedido_uuid = %pedido_uuid, "pagamento_usecase: pedido marcado como pago com sucesso");
        Ok(())
    }
}
