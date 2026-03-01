use std::sync::Arc;

use chrono::Datelike;
use uuid::Uuid;

use crate::models::{Pedido, StatusCupom, TipoCalculoPedido, calcular_preco_por_partes};
use crate::repositories::{ConfiguracaoPedidosLojaRepository, CupomRepository, PedidoRepository, PromocaoRepository, Repository as _};
use crate::utils::agora;

pub struct PedidoService {
    pedido_repo: Arc<PedidoRepository>,
    config_repo: Arc<ConfiguracaoPedidosLojaRepository>,
    cupom_repo: Arc<CupomRepository>,
    promocao_repo: Arc<PromocaoRepository>,
}

impl PedidoService {
    pub fn new(
        pedido_repo: Arc<PedidoRepository>,
        config_repo: Arc<ConfiguracaoPedidosLojaRepository>,
        cupom_repo: Arc<CupomRepository>,
        promocao_repo: Arc<PromocaoRepository>,
    ) -> Self {
        Self { pedido_repo, config_repo, cupom_repo, promocao_repo }
    }

    pub async fn salvar(&self, pedido: &Pedido) -> Result<Uuid, String> {
        self.pedido_repo.criar(pedido).await
    }

    /// Calcula e exibe os preços baseado na configuração da loja
    pub async fn processar_e_exibir_precos(
        &self,
        pedido: &mut Pedido,
        loja_uuid: uuid::Uuid
    ) -> Result<(), String> {
        let config_loja = self
            .config_repo
            .buscar_por_loja(loja_uuid)
            .await?
            .unwrap();

        println!("--- Processando Pedido {} ---", pedido.uuid);
        for item in &pedido.itens {
            let preco_media = calcular_preco_por_partes(
                &item.partes, &TipoCalculoPedido::MediaPonderada
            );
            let preco_caro = calcular_preco_por_partes(
                &item.partes, &TipoCalculoPedido::MaisCaro
            );
            let preco_loja = calcular_preco_por_partes(
                &item.partes, &config_loja.tipo_calculo
            );

            println!(
                "Item: Média: {:.2} | Mais caro: {:.2} | Loja Config: {:.2}",
                preco_media,
                preco_caro,
                preco_loja
            );
        }
        Ok(())
    }

    pub async fn buscar_completo(
        &self,
        pedido_uuid: uuid::Uuid
    ) -> Result<Option<Pedido>, String> {
        self.pedido_repo.buscar_completo(pedido_uuid).await
    }

    pub async fn listar(&self) -> Result<Vec<Pedido>, String> {
        self.pedido_repo.listar_todos().await
    }

    /// Método principal que orquestra o cálculo de preço, promoções e cupons
    pub async fn processar_e_finalizar_pedido(
        &self,
        pedido: &mut Pedido,
        codigo_cupom: Option<String>,
    ) -> Result<(), String> {
        
        // 1. Buscar configuração da loja (como calcular preço dos sabores)
        let config_loja = self.config_repo
            .buscar_por_loja(pedido.loja_uuid)
            .await?
            .ok_or("Configuração da loja não encontrada")?;

        // 2. Calcular Subtotal dos Itens
        // Nota: Em um cenário real, buscaríamos preços atualizados do DB.
        // Aqui usamos os preços que já vieram no objeto Pedido (snapshots).
        let mut subtotal_calculado = 0.0;
        
        for item in &pedido.itens {
            // Soma o preço base do item (calculado pela regra de sabores)
            let preco_item = calcular_preco_por_partes(
                &item.partes,
                &config_loja.tipo_calculo
            );

            // Soma adicionais
            let total_adicionais: f64 = item.partes.iter()
                .flat_map(|p| &p.adicionais)
                .map(|a| a.preco)
                .sum();

            subtotal_calculado += (preco_item + total_adicionais) * item.quantidade as f64;
        }

        pedido.subtotal = subtotal_calculado;
        
        // 3. Calcular descontos
        let (desconto_promocao, descricao_promo) =
            self.calcular_melhor_promocao(pedido).await?;

        let (desconto_cupom, descricao_cupom) = self.validar_cupom(
            pedido,
            codigo_cupom
        ).await?;

        // 4. Decisão de negócio: Escolher o maior desconto (não acumulativo)
        // Ou aplicar lógica de prioridade. Ex: Cupom tem prioridade, senão usa promoção.
        
        let desconto_final;
        let observacao_desconto;

        if desconto_cupom > 0.0 {
            desconto_final = desconto_cupom;
            observacao_desconto = format!("Cupom aplicado: {}", descricao_cupom);
            // Aqui você poderia marcar o cupom como usado no banco
        } else if desconto_promocao > 0.0 {
            desconto_final = desconto_promocao;
            observacao_desconto = format!("Promoção aplicada: {}", descricao_promo);
        } else {
            desconto_final = 0.0;
            observacao_desconto = "Nenhum desconto aplicado".to_string();
        }

        pedido.desconto = Some(desconto_final);
        pedido.total = pedido.subtotal + pedido.taxa_entrega - desconto_final;

        // Atualiza observações do pedido com info do desconto
        if let Some(mut obs) = pedido.observacoes.clone() {
            obs.push_str(&format!(" | {}", observacao_desconto));
            pedido.observacoes = Some(obs);
        } else {
            pedido.observacoes = Some(observacao_desconto);
        }

        println!("Pedido processado: Subtotal {:.2} | Desconto {:.2} | Total {:.2}", 
            pedido.subtotal,
            desconto_final,
            pedido.total
        );

        self.salvar(pedido).await?;

        Ok(())
    }




    /// Lógica para verificar promoções ativas da loja
    async fn calcular_melhor_promocao(
        &self,
        pedido: &Pedido
    ) -> Result<(f64, String), String> {
        // Assumindo que existe um método buscar_ativas no repo (ou listar_todos filtrado)
        // Para simplificar, vamos simular a busca:
    

        // Ideal: filtrar por loja_uuid e status ativo
        let promocoes = self.promocao_repo.listar_todos().await?; 
        let agora = agora(); // String de data hora atual
        
        // Helper simples para obter dia da semana (0=Domingo, 6=Sábado)
        // Nota: Em produção, use chrono para parsing correto da string
        let dia_semana_atual = chrono::Utc::now().weekday().num_days_from_sunday() as u8;

        let mut melhor_desconto = 0.0;
        let mut melhor_descricao = String::new();

        for promo in promocoes {
            if promo.loja_uuid != pedido.loja_uuid { continue; }
            
            // Usa o método eh_aplicavel do modelo Promocao
            if promo.eh_aplicavel(
                pedido.subtotal,
                agora.clone(),
                dia_semana_atual
            ) {

                let valor_desc = promo.calcular_desconto(
                    pedido.subtotal,
                    pedido.taxa_entrega
                );

                if valor_desc > melhor_desconto {
                    melhor_desconto = valor_desc;
                    melhor_descricao = promo.nome;
                }
            }
        }

        Ok((melhor_desconto, melhor_descricao))
    }


    /// Lógica para validar e calcular cupom
    async fn validar_cupom(
        &self,
        pedido: &Pedido,
        codigo: Option<String>
    ) -> Result<(f64, String), String> {

        if let Some(cod) = codigo {
            // Busca cupom pelo código
            if let Some(cupom) = self.cupom_repo.buscar_por_codigo(&cod).await? {

                // Validações básicas
                if cupom.loja_uuid != pedido.loja_uuid {
                    return Ok((0.0, "Cupom inválido para esta loja".to_string()));
                }

                if cupom.status != StatusCupom::Ativo {
                    return Ok((0.0, "Cupom inativo".to_string()));
                }

                // Verifica validade (simples comparação de string ISO 8601
                // funciona se formato for igual)
                if agora() > cupom.data_validade {
                     return Ok((0.0, "Cupom expirado".to_string()));
                }
                // Verifica valor mínimo
                if let Some(minimo) = cupom.valor_minimo {
                    if pedido.subtotal < minimo {
                        return Ok((0.0, format!("Pedido abaixo do mínimo de {:.2}", minimo)));
                    }
                }

                let desconto = cupom.calcular_desconto(
                    pedido.subtotal,
                    pedido.taxa_entrega
                );

                return Ok((desconto, cupom.codigo));
            }
        }
        Ok((0.0, String::new()))
    }


}