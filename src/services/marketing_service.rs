use std::sync::Arc;

use uuid::Uuid;
use rust_decimal::Decimal;

use crate::entities::cupom::Model as Cupom;
use crate::entities::promocao::Model as Promocao;
use crate::entities::avaliacao_loja::Model as AvaliacaoDeLoja;
use crate::entities::avaliacao_produto::Model as AvaliacaoDeProduto;

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

#[allow(dead_code)]
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
        valor_desconto: Option<Decimal>,
        valor_minimo: Option<Decimal>,
        data_validade: String,
        limite_uso: Option<i32>,
    ) -> Result<Cupom, String> {

        let cupom = Cupom {
            uuid: Uuid::new_v4(),
            loja_uuid,
            codigo: codigo.to_uppercase(),
            descricao,
            tipo_desconto,
            valor_desconto,
            valor_minimo,
            data_validade,
            limite_uso,
            uso_atual: 0,
            status: "Ativo".to_string(),
            criado_em: chrono::Utc::now(),
        };

        self.cupom_repo.criar(&cupom).await?;

        Ok(cupom)
    }

    pub async fn criar_promocao(
        &self,
        loja_uuid: Uuid,
        nome: String,
        descricao: String,
        tipo_desconto: String,
        valor_desconto: Option<Decimal>,
        valor_minimo: Option<Decimal>,
        data_inicio: String,
        data_fim: String,
        dias_semana_validos: Option<Vec<u8>>,
        tipo_escopo: String,
        produto_uuid: Option<Uuid>,
        categoria_uuid: Option<Uuid>,
        prioridade: i32,
    ) -> Result<Promocao, String> {

        let dias_semana_i32 = dias_semana_validos.map(|v| v.into_iter().map(|d| d as i32).collect());

        let promocao = Promocao {
            uuid: Uuid::new_v4(),
            loja_uuid,
            nome,
            descricao,
            tipo_desconto,
            valor_desconto,
            valor_minimo,
            data_inicio,
            data_fim,
            dias_semana_validos: dias_semana_i32,
            tipo_escopo,
            produto_uuid,
            categoria_uuid,
            status: "Ativo".to_string(),
            prioridade,
            criado_em: chrono::Utc::now(),
        };

        self.promocao_repo.criar(&promocao).await?;

        Ok(promocao)
    }

    pub async fn avaliar_loja(
        &self,
        loja_uuid: Uuid,
        usuario_uuid: Uuid,
        nota: Decimal,
        comentario: Option<String>,
    ) -> Result<AvaliacaoDeLoja, String> {

        let avaliacao = AvaliacaoDeLoja {
            uuid: Uuid::new_v4(),
            loja_uuid,
            usuario_uuid,
            nota,
            comentario,
            criado_em: chrono::Utc::now(),
        };

        self.avaliacao_loja_repo.criar(&avaliacao).await?;

        Ok(avaliacao)
    }

    pub async fn avaliar_produto(
        &self,
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
        produto_uuid: Uuid,
        comentario: Option<String>,
        nota: Decimal,
        descricao: String,
    ) -> Result<AvaliacaoDeProduto, String> {

        let avaliacao = AvaliacaoDeProduto {
            uuid: Uuid::new_v4(),
            usuario_uuid,
            loja_uuid,
            produto_uuid,
            nota,
            descricao,
            comentario,
            criado_em: chrono::Utc::now(),
        };

        self.avaliacao_prod_repo.criar(&avaliacao).await?;

        Ok(avaliacao)
    }

    pub async fn listar_cupons(&self, loja_uuid: Uuid) -> Result<Vec<Cupom>, String> {
        self.cupom_repo.listar_todos_por_loja(loja_uuid).await
    }

    pub async fn atualizar_cupom(
        &self,
        uuid: Uuid,
        _loja_uuid: Uuid,
        codigo: String,
        descricao: String,
        tipo_desconto: String,
        valor_desconto: Option<Decimal>,
        valor_minimo: Option<Decimal>,
        data_validade: String,
        limite_uso: Option<i32>,
    ) -> Result<(), String> {
        let mut cupom = self.cupom_repo.buscar_por_uuid(uuid).await?
            .ok_or("Cupom não encontrado")?;
        cupom.codigo = codigo.to_uppercase();
        cupom.descricao = descricao;
        cupom.tipo_desconto = tipo_desconto;
        cupom.valor_desconto = valor_desconto;
        cupom.valor_minimo = valor_minimo;
        cupom.data_validade = data_validade;
        cupom.limite_uso = limite_uso;
        self.cupom_repo.atualizar(cupom).await
    }

    pub async fn deletar_cupom(&self, uuid: Uuid) -> Result<(), String> {
        self.cupom_repo.deletar(uuid).await
    }

    pub async fn listar_promocoes(&self, loja_uuid: Uuid) -> Result<Vec<Promocao>, String> {
        self.promocao_repo.listar_todos_por_loja(loja_uuid).await
    }

    pub async fn atualizar_promocao(
        &self,
        uuid: Uuid,
        loja_uuid: Uuid,
        nome: String,
        descricao: String,
        tipo_desconto: String,
        valor_desconto: Option<Decimal>,
        valor_minimo: Option<Decimal>,
        data_inicio: String,
        data_fim: String,
        dias_semana_validos: Option<Vec<u8>>,
        tipo_escopo: String,
        produto_uuid: Option<Uuid>,
        categoria_uuid: Option<Uuid>,
        prioridade: i32,
    ) -> Result<(), String> {
        let dias_semana_i32 = dias_semana_validos.map(|v| v.into_iter().map(|d| d as i32).collect());

        let mut promocao = Promocao {
            uuid,
            loja_uuid,
            nome,
            descricao,
            tipo_desconto,
            valor_desconto,
            valor_minimo,
            data_inicio,
            data_fim,
            dias_semana_validos: dias_semana_i32,
            tipo_escopo,
            produto_uuid,
            categoria_uuid,
            status: "Ativo".to_string(),
            prioridade,
            criado_em: chrono::Utc::now(),
        };

        self.promocao_repo.atualizar(promocao).await
    }

    pub async fn deletar_promocao(&self, uuid: Uuid) -> Result<(), String> {
        self.promocao_repo.deletar(uuid).await
    }
}
