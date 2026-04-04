use std::sync::Arc;

use uuid::Uuid;

use crate::models::{Cupom, Promocao, AvaliacaoDeLoja, AvaliacaoDeProduto};
use crate::repositories::{
    CupomRepository,
    PromocaoRepository,
    AvaliacaoDeLojaRepository,
    AvaliacaoDeProdutoRepository,
    Repository as _
};

#[derive(Clone)]
pub struct MarketingService {
    cupom_repo: Arc<CupomRepository>,
    promocao_repo: Arc<PromocaoRepository>,
    avaliacao_loja_repo: Arc<AvaliacaoDeLojaRepository>,
    avaliacao_prod_repo: Arc<AvaliacaoDeProdutoRepository>,
}

impl MarketingService {
    pub fn new(
        cupom_repo: Arc<CupomRepository>,
        promocao_repo: Arc<PromocaoRepository>,
        avaliacao_loja_repo: Arc<AvaliacaoDeLojaRepository>,
        avaliacao_prod_repo: Arc<AvaliacaoDeProdutoRepository>,
    ) -> Self {
        Self { cupom_repo, promocao_repo, avaliacao_loja_repo, avaliacao_prod_repo }
    }

    pub async fn criar_cupom(
        &self,
        loja_uuid: Uuid,
        codigo: String,
        descricao: String,
        tipo_desconto: String,
        valor_desconto: Option<f64>,
        valor_minimo: Option<f64>,
        data_validade: String,
        limite_uso: Option<i32>,
    ) -> Result<Cupom, String> {

        let cupom = Cupom::new(
            loja_uuid,
            codigo,
            descricao,
            tipo_desconto,
            valor_desconto,
            valor_minimo,
            data_validade,
            limite_uso
        );

        self.cupom_repo.criar(&cupom).await?;

        Ok(cupom)
    }

    pub async fn criar_promocao(
        &self,
        loja_uuid: Uuid,
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

        let promocao = Promocao::new(
            loja_uuid,
            nome,
            descricao,
            tipo_desconto,
            valor_desconto,
            valor_minimo,
            data_inicio,
            data_fim,
            dias_semana_validos,
            prioridade
        );

        let _ = self.promocao_repo.criar(&promocao).await;

        Ok(promocao)
    }

    pub async fn avaliar_loja(
        &self,
        loja_uuid: Uuid,
        usuario_uuid: Uuid,
        nota: f64,
        comentario: Option<String>,
    ) -> Result<AvaliacaoDeLoja, String> {

        let avaliacao: AvaliacaoDeLoja = AvaliacaoDeLoja::new(
            loja_uuid,
            usuario_uuid,
            nota,
            comentario
        );

        self.avaliacao_loja_repo.criar(&avaliacao).await?;

        Ok(avaliacao)
    }

    pub async fn avaliar_produto(
        &self,
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
        produto_uuid: Uuid,
        comentario: Option<String>,
        nota: f64,
        descricao: String,
    ) -> Result<AvaliacaoDeProduto, String> {

        let avaliacao: AvaliacaoDeProduto = AvaliacaoDeProduto::new(
            usuario_uuid,
            loja_uuid,
            produto_uuid,
            comentario,
            nota,
            descricao
        );

        self.avaliacao_prod_repo.criar(&avaliacao).await?;

        Ok(avaliacao)
    }
    
    pub async fn listar_cupons(&self) -> Result<Vec<Cupom>, String> {
        self.cupom_repo.listar_todos().await
    }
}