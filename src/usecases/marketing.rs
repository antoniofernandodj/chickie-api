

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{AvaliacaoDeLoja, AvaliacaoDeProduto, Cupom, Promocao, Usuario},
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

    pub async fn criar_promocao(
        &self,
        nome: String,
        descricao: String,
        tipo_desconto: String,
        valor_desconto: Option<f64>,
        valor_minimo: Option<f64>,
        data_inicio: String,
        data_fim: String,
        dias_semana_validos: Option<Vec<u8>>,
        prioridade: i32,
    ) -> Result<Promocao, String> {
        self.marketing_service.criar_promocao(
            self.loja_uuid,
            nome,
            descricao,
            tipo_desconto,
            valor_desconto,
            valor_minimo,
            data_inicio,
            data_fim,
            dias_semana_validos,
            prioridade
        ).await
    }

    pub async fn listar_cupons(&self) -> Result<Vec<Cupom>, String> {
        self.marketing_service.listar_cupons(self.loja_uuid).await
    }
}