use std::sync::Arc;

use uuid::Uuid;
use rust_decimal::Decimal;

use crate::models::{Cupom, Promocao, AvaliacaoDeLoja, AvaliacaoDeProduto};
use crate::ports::{
    CupomRepositoryPort,
    PromocaoRepositoryPort,
    AvaliacaoDeLojaRepositoryPort,
    AvaliacaoDeProdutoRepositoryPort,
};

#[derive(Clone)]
pub struct MarketingService {
    cupom_repo: Arc<dyn CupomRepositoryPort>,
    promocao_repo: Arc<dyn PromocaoRepositoryPort>,
    avaliacao_loja_repo: Arc<dyn AvaliacaoDeLojaRepositoryPort>,
    avaliacao_prod_repo: Arc<dyn AvaliacaoDeProdutoRepositoryPort>,
}

#[allow(dead_code)]
impl MarketingService {
    pub fn new(
        cupom_repo: Arc<dyn CupomRepositoryPort>,
        promocao_repo: Arc<dyn PromocaoRepositoryPort>,
        avaliacao_loja_repo: Arc<dyn AvaliacaoDeLojaRepositoryPort>,
        avaliacao_prod_repo: Arc<dyn AvaliacaoDeProdutoRepositoryPort>,
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
        let data_validade_parsed = chrono::DateTime::parse_from_rfc3339(&data_validade)
            .map_err(|e| format!("Invalid date format for data_validade: {}", e))?
            .with_timezone(&chrono::Utc);

        let cupom = Cupom::new(
            loja_uuid,
            codigo,
            descricao,
            tipo_desconto,
            valor_desconto,
            valor_minimo,
            data_validade_parsed,
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
            tipo_escopo,
            produto_uuid,
            categoria_uuid,
            prioridade
        );

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
        nota: Decimal,
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
    
    pub async fn listar_cupons(&self, loja_uuid: Uuid) -> Result<Vec<Cupom>, String> {
        self.cupom_repo.listar_por_loja(loja_uuid).await.map_err(|e| e.to_string())
    }

    pub async fn buscar_cupom(&self, uuid: Uuid) -> Result<Cupom, String> {
        self.cupom_repo.buscar_por_uuid(uuid).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Cupom não encontrado".to_string())
    }

    pub async fn listar_todos_cupons(&self) -> Result<Vec<Cupom>, String> {
        self.cupom_repo.listar_todos().await.map_err(|e| e.to_string())
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
        let data_validade_parsed = chrono::DateTime::parse_from_rfc3339(&data_validade)
            .map_err(|e| format!("Invalid date format for data_validade: {}", e))?
            .with_timezone(&chrono::Utc);

        let mut cupom = self.cupom_repo.buscar_por_uuid(uuid).await?
            .ok_or("Cupom não encontrado")?;
        cupom.codigo = codigo.to_uppercase();
        cupom.descricao = descricao;
        cupom.tipo_desconto = tipo_desconto;
        cupom.valor_desconto = valor_desconto;
        cupom.valor_minimo = valor_minimo;
        cupom.data_validade = data_validade_parsed;
        cupom.limite_uso = limite_uso;
        self.cupom_repo.atualizar(cupom).await.map_err(|e| e.to_string())
    }

    pub async fn deletar_cupom(&self, uuid: Uuid) -> Result<(), String> {
        self.cupom_repo.deletar(uuid).await.map_err(|e| e.to_string())
    }

    pub async fn listar_promocoes(&self, loja_uuid: Uuid) -> Result<Vec<Promocao>, String> {
        self.promocao_repo.listar_por_loja(loja_uuid).await.map_err(|e| e.to_string())
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
        let mut promocao = Promocao::new(
            loja_uuid,
            nome,
            descricao,
            tipo_desconto,
            valor_desconto,
            valor_minimo,
            data_inicio,
            data_fim,
            dias_semana_validos,
            tipo_escopo,
            produto_uuid,
            categoria_uuid,
            prioridade
        );
        promocao.uuid = uuid;

        self.promocao_repo.atualizar(promocao).await.map_err(|e| e.to_string())
    }

    pub async fn deletar_promocao(&self, uuid: Uuid) -> Result<(), String> {
        self.promocao_repo.deletar(uuid).await.map_err(|e| e.to_string())
    }
}