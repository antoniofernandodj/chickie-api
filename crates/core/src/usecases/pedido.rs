use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::{
    models::{ParteDeItemPedido, Pedido, EstadoDePedido, EnderecoEntrega, Produto, Usuario},
    services::{PedidoService, PedidoComEntrega},
    ports::PedidoCriado,
    ports::ProdutoRepositoryPort,
};

pub struct PedidoUsecase {
    pub pedido_service: Arc<PedidoService>,
    pub produto_repo: Arc<dyn ProdutoRepositoryPort>,
    pub usuario: Option<Usuario>,
    pub loja_uuid: Uuid,
}


impl PedidoUsecase {
    pub fn new(
        pedido_service: Arc<PedidoService>,
        produto_repo: Arc<dyn ProdutoRepositoryPort>,
        usuario: Option<Usuario>,
        loja_uuid: Uuid,
    ) -> Self {

        Self {
            pedido_service,
            produto_repo,
            usuario,
            loja_uuid
        }

    }

    pub async fn criar_pedido(
        &self,
        taxa_entrega: Decimal,
        forma_pagamento: String,
        observacoes: Option<String>,
        contato: Option<String>,
        codigo_cupom: Option<String>,
        itens: Vec<ItemPedidoInput>,
        endereco_entrega: Option<EnderecoEntregaInput>,
    ) -> Result<PedidoCriado, String> {

        tracing::info!(
            target: "pedido",
            "[USECASE] criar_pedido iniciado — loja={} usuario={:?}",
            self.loja_uuid,
            self.usuario.as_ref().map(|u| u.uuid),
        );

        // 1. Validar produtos e montar partes
        let mut partes_por_item: Vec<Vec<ParteDeItemPedido>> = Vec::new();

        for item_req in &itens {
            let mut partes_item = Vec::new();
            for parte_req in &item_req.partes {
                tracing::debug!(target: "pedido", "[USECASE] buscando produto uuid={}", parte_req.produto_uuid);
                let produto: Produto = self.produto_repo
                    .buscar_por_uuid(parte_req.produto_uuid)
                    .await
                    .map_err(|e| e.to_string())?
                    .ok_or_else(|| format!("Produto {} não encontrado", parte_req.produto_uuid))?;

                tracing::debug!(target: "pedido", "[USECASE] produto encontrado: nome={} preco={}", produto.nome, produto.preco);
                partes_item.push(ParteDeItemPedido::new(
                    produto.uuid,
                    produto.nome.clone(),
                    produto.loja_uuid,
                    produto.preco,
                    parte_req.posicao,
                ));
            }
            partes_por_item.push(partes_item);
        }

        // 2. Criar pedido base
        let usuario_uuid = self.usuario.as_ref().map(|u| u.uuid);
        let contato_filtrado = contato.map(|c| c.chars().filter(|ch| ch.is_ascii_digit()).collect::<String>()).filter(|s| !s.is_empty());
        let mut pedido = Pedido::new(
            usuario_uuid,
            self.loja_uuid,
            Decimal::ZERO,
            taxa_entrega,
            forma_pagamento,
            observacoes,
            contato_filtrado,
        );

        tracing::info!(
            target: "pedido",
            "[USECASE] pedido instanciado uuid={} loja={} usuario={:?}",
            pedido.uuid, pedido.loja_uuid, pedido.usuario_uuid,
        );

        // 3. Adicionar itens com partes
        let mut partes_iter = partes_por_item.into_iter();
        for item_req in itens {
            if let Some(partes_item) = partes_iter.next() {
                pedido.adicionar_item(
                    item_req.quantidade,
                    item_req.observacoes,
                    partes_item
                );
            }
        }

        tracing::info!(
            target: "pedido",
            "[USECASE] itens montados: {} itens no pedido uuid={}",
            pedido.itens.len(), pedido.uuid,
        );

        // 4. Montar endereço de entrega (opcional)
        let endereco: Option<EnderecoEntrega> = endereco_entrega.map(|e| EnderecoEntrega::new(
            Uuid::nil(),
            self.loja_uuid,
            e.cep,
            e.logradouro,
            e.numero,
            e.complemento,
            e.bairro,
            e.cidade,
            e.estado,
        ));

        tracing::info!(
            target: "pedido",
            "[USECASE] chamando service.criar_pedido_com_entrega uuid={} tem_endereco={}",
            pedido.uuid, endereco.is_some(),
        );

        // 5. Salvar via service
        let result = self.pedido_service
            .criar_pedido_com_entrega(&mut pedido, endereco, codigo_cupom)
            .await;

        match &result {
            Ok(criado) => tracing::info!(target: "pedido", "[USECASE] service retornou uuid={} codigo={}", criado.uuid, criado.codigo),
            Err(e) => tracing::error!(target: "pedido", "[USECASE] service retornou erro: {}", e),
        }

        result
    }

    pub async fn buscar_por_codigo(&self, codigo: &str) -> Result<Pedido, String> {
        self.pedido_service.buscar_por_codigo(codigo).await
    }

    pub async fn listar_por_loja(&self) -> Result<Vec<Pedido>, String> {
        self.pedido_service.listar_por_loja(self.loja_uuid).await
    }

    pub async fn listar_por_usuario(&self) -> Result<Vec<Pedido>, String> {
        let uuid = self.usuario
            .as_ref()
            .map(|u| u.uuid)
            .ok_or_else(|| "Usuário não autenticado".to_string())?;
        self.pedido_service.listar_por_usuario(uuid).await
    }

    pub async fn buscar_pedido_com_entrega(
        &self,
        pedido_uuid: Uuid,
    ) -> Result<PedidoComEntrega, String> {
        self.pedido_service.buscar_pedido_com_entrega(pedido_uuid, self.loja_uuid).await
    }

    pub async fn atualizar_status_pedido(
        &self,
        pedido_uuid: Uuid,
        novo_status: EstadoDePedido,
    ) -> Result<Pedido, String> {
        self.pedido_service.atualizar_status(pedido_uuid, novo_status).await
    }

    pub async fn atribuir_entregador(
        &self,
        pedido_uuid: Uuid,
        entregador_uuid: Uuid,
    ) -> Result<(), String> {
        self.pedido_service.atribuir_entregador(pedido_uuid, entregador_uuid).await
    }

    pub async fn remover_entregador(
        &self,
        pedido_uuid: Uuid,
    ) -> Result<(), String> {
        self.pedido_service.remover_entregador(pedido_uuid).await
    }
}

// ─── Input types ───

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemPedidoInput {
    pub quantidade: i32,
    pub observacoes: Option<String>,
    pub partes: Vec<ParteItemInput>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParteItemInput {
    pub produto_uuid: Uuid,
    pub posicao: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnderecoEntregaInput {
    pub cep: Option<String>,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
}
