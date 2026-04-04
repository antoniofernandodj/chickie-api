

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{AvaliacaoDeLoja, AvaliacaoDeProduto, Cupom, UsoCupom, Promocao, Usuario},
    services::MarketingService
};

pub struct MarketingUsecase {
    pub marketing_service: Arc<MarketingService>,
    pub loja_uuid: Uuid,
    pub usuario: Usuario,
}


impl MarketingUsecase {
    pub fn new(
        marketing_service: Arc<MarketingService>,
        loja_uuid: Uuid,
        usuario: Usuario
    ) -> Self {

        Self {
            marketing_service,
            loja_uuid,
            usuario
        }

    }

    pub async fn avaliar_loja(
        &self,
        nota: f64,
        comentario: Option<String>,
    ) -> Result<AvaliacaoDeLoja, String> {
        self.marketing_service.avaliar_loja(
            self.loja_uuid,
            self.usuario.uuid,
            nota,
            comentario
        ).await
    }

    pub async fn avaliar_produto(
        &self,
        produto_uuid: Uuid,
        nota: f64,
        descricao: String,
        comentario: Option<String>,
    ) -> Result<AvaliacaoDeProduto, String> {
        self.marketing_service.avaliar_produto(
            self.usuario.uuid,
            self.loja_uuid,
            produto_uuid,
            comentario,
            nota,
            descricao
        ).await
    }
}