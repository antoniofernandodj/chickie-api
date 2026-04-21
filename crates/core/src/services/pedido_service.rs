use std::sync::Arc;

use chrono::{Datelike, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::models::{Pedido, EstadoDePedido, StatusCupom, calcular_preco_por_partes};
use crate::ports::{ConfiguracaoPedidosLojaRepositoryPort, CupomRepositoryPort, EnderecoEntregaRepositoryPort, PedidoRepositoryPort, PromocaoRepositoryPort, PedidoComEntregador};


use crate::models::EnderecoEntrega;

/// DTO para retorno de pedido com endereço de entrega
pub struct PedidoComEntrega {
    pub pedido: Pedido,
    pub endereco_entrega: Option<EnderecoEntrega>,
}

/// Dados de entrada para criar endereço de entrega (vindo do request HTTP)
pub struct DadosEnderecoEntrega {
    pub cep: Option<String>,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    // pub latitude: Option<f64>,
    // pub longitude: Option<f64>,
}

#[allow(dead_code)]
impl DadosEnderecoEntrega {
    pub fn to_endereco_entrega(
        self,
        pedido_uuid: Uuid,
        loja_uuid: Uuid,
    ) -> EnderecoEntrega {
        EnderecoEntrega::new(
            pedido_uuid,
            loja_uuid,
            self.cep,
            self.logradouro,
            self.numero,
            self.complemento,
            self.bairro,
            self.cidade,
            self.estado,
            // self.latitude,
            // self.longitude,
        )
    }
}

#[derive(Clone)]
pub struct PedidoService {
    pedido_repo: Arc<dyn PedidoRepositoryPort>,
    config_repo: Arc<dyn ConfiguracaoPedidosLojaRepositoryPort>,
    cupom_repo: Arc<dyn CupomRepositoryPort>,
    promocao_repo: Arc<dyn PromocaoRepositoryPort>,
    endereco_entrega_repo: Arc<dyn EnderecoEntregaRepositoryPort>,
}

impl PedidoService {
    pub fn new(
        pedido_repo: Arc<dyn PedidoRepositoryPort>,
        config_repo: Arc<dyn ConfiguracaoPedidosLojaRepositoryPort>,
        cupom_repo: Arc<dyn CupomRepositoryPort>,
        promocao_repo: Arc<dyn PromocaoRepositoryPort>,
        endereco_entrega_repo: Arc<dyn EnderecoEntregaRepositoryPort>,
    ) -> Self {
        Self { pedido_repo, config_repo, cupom_repo, promocao_repo, endereco_entrega_repo }
    }

    pub async fn salvar(&self, pedido: &Pedido) -> Result<Uuid, String> {
        self.pedido_repo.criar(pedido).await.map_err(|e| e.to_string())
    }

    pub async fn listar_por_loja(&self, loja_uuid: uuid::Uuid) -> Result<Vec<Pedido>, String> {
        self.pedido_repo.buscar_completos_por_loja(loja_uuid).await.map_err(|e| e.to_string())
    }

    pub async fn listar_por_usuario(&self, usuario_uuid: uuid::Uuid) -> Result<Vec<Pedido>, String> {
        self.pedido_repo.buscar_completos_por_usuario(usuario_uuid).await.map_err(|e| e.to_string())
    }

    /// Lógica para verificar promoções ativas da loja
    async fn __calcular_melhor_promocao(
        &self,
        pedido: &Pedido
    ) -> Result<(Decimal, String), String> {
        // Ideal: filtrar por loja_uuid e status ativo
        let promocoes = self.promocao_repo.listar_todos().await?;
        let agora = chrono::Utc::now();

        // Helper simples para obter dia da semana (0=Domingo, 6=Sábado)
        let dia_semana_atual = chrono::Utc::now().weekday().num_days_from_sunday() as u8;

        let mut melhor_desconto = Decimal::ZERO;
        let mut melhor_descricao = String::new();

        for promo in promocoes {
            if promo.loja_uuid != pedido.loja_uuid { continue; }

            // Usa o método eh_aplicavel do modelo Promocao
            if promo.eh_aplicavel(
                pedido.subtotal,
                agora,
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
    ) -> Result<(Decimal, String), String> {

        if let Some(cod) = codigo {
            // Busca cupom pelo código
            if let Some(cupom) = self.cupom_repo.buscar_por_codigo(&cod, pedido.loja_uuid).await? {

                // Validações básicas
                if cupom.loja_uuid != pedido.loja_uuid {
                    return Ok((Decimal::ZERO, "Cupom inválido para esta loja".to_string()));
                }

                if cupom.status != StatusCupom::Ativo {
                    return Ok((Decimal::ZERO, "Cupom inativo".to_string()));
                }

                // Verifica validade
                if Utc::now() > cupom.data_validade {
                     return Ok((Decimal::ZERO, "Cupom expirado".to_string()));
                }
                // Verifica valor mínimo
                if let Some(minimo) = cupom.valor_minimo {
                    if pedido.subtotal < minimo {
                        return Ok((Decimal::ZERO, format!("Pedido abaixo do mínimo de {:.2}", minimo)));
                    }
                }

                let desconto = cupom.calcular_desconto(
                    pedido.subtotal,
                    pedido.taxa_entrega
                );

                return Ok((desconto, cupom.codigo));
            }
        }
        Ok((Decimal::ZERO, String::new()))
    }

    /// Método principal para criar pedido, com endereço de entrega opcional
    pub async fn criar_pedido_com_entrega(
        &self,
        pedido: &mut Pedido,
        endereco_entrega: Option<crate::models::EnderecoEntrega>,
        codigo_cupom: Option<String>,
    ) -> Result<Uuid, String> {

        tracing::info!(
            target: "pedido",
            "[SERVICE] criar_pedido_com_entrega iniciado uuid={} loja={} itens={}",
            pedido.uuid, pedido.loja_uuid, pedido.itens.len(),
        );

        // 1. Processar preços e descontos (ATENÇÃO: também chama self.salvar internamente)
        tracing::info!(target: "pedido", "[SERVICE] chamando __processar_e_finalizar_pedido uuid={}", pedido.uuid);
        self.__processar_e_finalizar_pedido(pedido, codigo_cupom).await?;
        tracing::info!(
            target: "pedido",
            "[SERVICE] __processar_e_finalizar_pedido concluido uuid={} subtotal={} total={} desconto={:?}",
            pedido.uuid, pedido.subtotal, pedido.total, pedido.desconto,
        );

        // 2. Salvar o pedido no banco (retorna UUID)
        tracing::warn!(
            target: "pedido",
            "[SERVICE] chamando pedido_repo.criar SEGUNDA VEZ uuid={} — isso causará duplicate key se __processar já salvou",
            pedido.uuid,
        );
        let pedido_uuid = self.pedido_repo.criar(pedido).await?;
        tracing::info!(target: "pedido", "[SERVICE] pedido_repo.criar retornou uuid={}", pedido_uuid);

        // 3. Criar endereço de entrega se fornecido
        if let Some(mut endereco) = endereco_entrega {
            endereco.uuid = Uuid::new_v4();
            endereco.pedido_uuid = pedido_uuid;
            endereco.loja_uuid = pedido.loja_uuid;
            tracing::info!(target: "pedido", "[SERVICE] criando endereco_entrega para pedido uuid={}", pedido_uuid);
            let result = self.endereco_entrega_repo.criar(&endereco).await;
            match &result {
                Ok(_) => tracing::info!(target: "pedido", "[SERVICE] endereco_entrega criado para pedido uuid={}", pedido_uuid),
                Err(e) => tracing::error!(target: "pedido", "[SERVICE] erro ao criar endereco_entrega para pedido uuid={}: {}", pedido_uuid, e),
            }
            result?;
        }

        tracing::info!(target: "pedido", "[SERVICE] criar_pedido_com_entrega concluido uuid={}", pedido_uuid);
        Ok(pedido_uuid)
    }


    /// Busca um pedido completo COM endereço de entrega
    pub async fn buscar_pedido_com_entrega(
        &self,
        pedido_uuid: Uuid,
        _loja_uuid: uuid::Uuid
    ) -> Result<PedidoComEntrega, String> {

        let pedido = self.pedido_repo.buscar_completo(pedido_uuid).await.map_err(|e| e.to_string())?
            .ok_or("Pedido não encontrado")?;

        let endereco_entrega = self.endereco_entrega_repo
            .buscar_por_pedido(pedido_uuid)
            .await?;

        Ok(PedidoComEntrega {
            pedido,
            endereco_entrega,
        })
    }



    /// Método principal que orquestra o cálculo de preço, promoções e cupons
    async fn __processar_e_finalizar_pedido(
        &self,
        pedido: &mut Pedido,
        codigo_cupom: Option<String>,
    ) -> Result<(), String> {

        tracing::info!(
            target: "pedido",
            "[SERVICE] __processar_e_finalizar_pedido iniciado uuid={} loja={}",
            pedido.uuid, pedido.loja_uuid,
        );

        // 1. Buscar configuração da loja (como calcular preço dos sabores)
        tracing::debug!(target: "pedido", "[SERVICE] buscando config da loja={}", pedido.loja_uuid);
        let config_loja = self.config_repo
            .buscar_por_loja(pedido.loja_uuid)
            .await?
            .ok_or("Configuração da loja não encontrada")?;
        tracing::debug!(target: "pedido", "[SERVICE] config loja encontrada tipo_calculo={:?}", config_loja.tipo_calculo);

        // 2. Calcular Subtotal dos Itens
        // Nota: Em um cenário real, buscaríamos preços atualizados do DB.
        // Aqui usamos os preços que já vieram no objeto Pedido (snapshots).
        let mut subtotal_calculado = Decimal::ZERO;

        for item in &pedido.itens {
            // Soma o preço base do item (calculado pela regra de sabores)
            let preco_item = calcular_preco_por_partes(
                &item.partes,
                &config_loja.tipo_calculo
            );

            // Soma adicionais
            let total_adicionais: Decimal = item.partes.iter()
                .flat_map(|p| &p.adicionais)
                .map(|a| a.preco)
                .sum();

            subtotal_calculado += (preco_item + total_adicionais) * Decimal::from(item.quantidade);
        }

        pedido.subtotal = subtotal_calculado;

        // 3. Calcular descontos
        let (desconto_promocao, descricao_promo) =
            self.__calcular_melhor_promocao(pedido).await?;

        let (desconto_cupom, descricao_cupom) = self.validar_cupom(
            pedido,
            codigo_cupom
        ).await?;

        // 4. Decisão de negócio: Escolher o maior desconto (não acumulativo)
        // Ou aplicar lógica de prioridade. Ex: Cupom tem prioridade, senão usa promoção.

        let desconto_final;
        let observacao_desconto;

        if desconto_cupom > Decimal::ZERO {
            desconto_final = desconto_cupom;
            observacao_desconto = format!("Cupom aplicado: {}", descricao_cupom);
            // Aqui você poderia marcar o cupom como usado no banco
        } else if desconto_promocao > Decimal::ZERO {
            desconto_final = desconto_promocao;
            observacao_desconto = format!("Promoção aplicada: {}", descricao_promo);
        } else {
            desconto_final = Decimal::ZERO;
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

        tracing::info!(
            target: "pedido",
            "[SERVICE] pedido processado uuid={} subtotal={:.2} desconto={:.2} total={:.2}",
            pedido.uuid, pedido.subtotal, desconto_final, pedido.total,
        );

        tracing::warn!(
            target: "pedido",
            "[SERVICE] chamando self.salvar (INSERT) dentro de __processar uuid={}",
            pedido.uuid,
        );
        self.salvar(pedido).await?;
        tracing::info!(target: "pedido", "[SERVICE] self.salvar concluido para uuid={}", pedido.uuid);

        Ok(())
    }

    /// Atualiza o status de um pedido para um novo estado válido
    pub async fn atualizar_status(
        &self,
        pedido_uuid: Uuid,
        novo_status: EstadoDePedido,
    ) -> Result<Pedido, String> {
        let mut pedido = self.pedido_repo.buscar_por_uuid(pedido_uuid).await?
            .ok_or("Pedido não encontrado")?;

        let status_atual = pedido.status.clone();

        if !status_atual.pode_transicionar_para(&novo_status) {
            return Err(format!(
                "Transição inválida: {:?} -> {:?}. Transições permitidas: {:?}",
                status_atual, novo_status, status_atual.transicoes_permitidas()
            ));
        }

        pedido.status = novo_status.clone();
        self.pedido_repo.atualizar(pedido.clone()).await?;

        tracing::info!(
            "Pedido {} atualizado: {:?} -> {:?}",
            pedido_uuid, status_atual, novo_status
        );

        Ok(pedido)
    }

    /// Atribui um entregador a um pedido
    pub async fn atribuir_entregador(
        &self,
        pedido_uuid: Uuid,
        entregador_uuid: Uuid,
    ) -> Result<(), String> {
        // Validar que o pedido existe
        self.pedido_repo.buscar_por_uuid(pedido_uuid).await?
            .ok_or("Pedido não encontrado")?;

        self.pedido_repo.atribuir_entregador(pedido_uuid, entregador_uuid).await?;

        tracing::info!(
            "Entregador {} atribuído ao pedido {}",
            entregador_uuid, pedido_uuid
        );

        Ok(())
    }

    /// Remove o entregador de um pedido
    pub async fn remover_entregador(
        &self,
        pedido_uuid: Uuid,
    ) -> Result<(), String> {
        self.pedido_repo.remover_entregador(pedido_uuid).await?;

        tracing::info!("Entregador removido do pedido {}", pedido_uuid);
        Ok(())
    }

    /// Busca pedido com informações do entregador
    pub async fn buscar_pedido_com_entregador(
        &self,
        pedido_uuid: Uuid,
    ) -> Result<PedidoComEntregador, String> {
        self.pedido_repo.buscar_com_entregador(pedido_uuid).await?
            .ok_or("Pedido não encontrado".to_string())
    }
}