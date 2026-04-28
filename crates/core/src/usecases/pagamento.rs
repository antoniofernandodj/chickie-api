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
        // 1. Buscar pedido
        let pedido = self.pedido_repo
            .buscar_por_uuid(pedido_uuid)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Pedido {} não encontrado", pedido_uuid))?;

        if pedido.pago {
            return Err("Pedido já foi pago".to_string());
        }

        // 2. Resolver nome, cpf e asaas_customer_id_cache
        let (nome, cpf, asaas_id_cache, usuario_uuid) = match usuario {
            Some(u) => (
                u.nome.clone(),
                u.cpf.clone(),
                u.asaas_customer_id.clone(),
                Some(u.uuid),
            ),
            None => {
                let p = pagador.ok_or(
                    "Usuário não autenticado — forneça nome e CPF no campo 'pagador'".to_string()
                )?;
                let cpf_limpo: String = p.cpf.chars().filter(|c| c.is_ascii_digit()).collect();
                (p.nome, cpf_limpo, None, None)
            }
        };

        // 3. Obter ou criar customer no Asaas
        let asaas_customer_id = match asaas_id_cache {
            Some(id) => id,
            None => {
                let id = self.asaas
                    .cadastrar_ou_buscar_usuario_no_asaas(&nome, &cpf)
                    .await?;
                // Persistir para evitar chamada repetida na próxima vez
                if let Some(uid) = usuario_uuid {
                    if let Err(e) = self.usuario_repo
                        .salvar_asaas_customer_id(uid, &id)
                        .await
                    {
                        tracing::warn!("Falha ao salvar asaas_customer_id usuario={}: {}", uid, e);
                    }
                }
                id
            }
        };

        // 4. Criar cobrança PIX
        let pagamento = self.asaas
            .criar_cobranca_pix(&asaas_customer_id, pedido.total, pedido_uuid)
            .await?;

        Ok(PagamentoOutput {
            payment_id: pagamento.payment_id,
            qr_code_image: pagamento.qr_code_image,
            pix_copia_cola: pagamento.pix_copia_cola,
            vencimento: pagamento.vencimento,
        })
    }

    /// Chamado pelo webhook do Asaas quando o pagamento é confirmado.
    pub async fn confirmar_pagamento(&self, pedido_uuid: Uuid) -> Result<(), String> {
        self.pedido_repo
            .marcar_como_pago(pedido_uuid)
            .await
            .map_err(|e| e.to_string())
    }
}
