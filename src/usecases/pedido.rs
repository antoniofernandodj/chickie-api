use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{ParteDeItemPedido, Pedido, EnderecoEntrega, Produto, Usuario},
    services::PedidoService,
    repositories::{ProdutoRepository, Repository as _}
};

pub struct PedidoUsecase {
    pub pedido_service: Arc<PedidoService>,
    pub produto_repo: Arc<ProdutoRepository>,
    pub usuario: Usuario,
    pub loja_uuid: Uuid,
}


impl PedidoUsecase {
    pub fn new(
        pedido_service: Arc<PedidoService>,
        produto_repo: Arc<ProdutoRepository>,
        usuario: Usuario,
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
        taxa_entrega: f64,
        forma_pagamento: String,
        observacoes: Option<String>,
        codigo_cupom: Option<String>,
        itens: Vec<ItemPedidoInput>,
        endereco_entrega: EnderecoEntregaInput,
    ) -> Result<Uuid, String> {

        // 1. Validar produtos e montar partes
        let mut partes_por_item: Vec<Vec<ParteDeItemPedido>> = Vec::new();

        for item_req in &itens {
            let mut partes_item = Vec::new();
            for parte_req in &item_req.partes {
                let produto: Produto = self.produto_repo
                    .buscar_por_uuid(parte_req.produto_uuid)
                    .await
                    .map_err(|e| e.to_string())?
                    .ok_or_else(|| format!("Produto {} não encontrado", parte_req.produto_uuid))?;

                partes_item.push(ParteDeItemPedido::new(&produto, parte_req.posicao));
            }
            partes_por_item.push(partes_item);
        }

        // 2. Criar pedido base
        let mut pedido = Pedido::new(
            self.usuario.uuid,
            self.loja_uuid,
            0.0,
            taxa_entrega,
            forma_pagamento,
            observacoes,
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

        // 4. Montar endereço de entrega
        let endereco: EnderecoEntrega = EnderecoEntrega::new(
            Uuid::nil(), // Será substituído pelo service
            self.loja_uuid,
            endereco_entrega.cep,
            endereco_entrega.logradouro,
            endereco_entrega.numero,
            endereco_entrega.complemento,
            endereco_entrega.bairro,
            endereco_entrega.cidade,
            endereco_entrega.estado,
        );

        // 5. Salvar via service
        self.pedido_service
            .criar_pedido_com_entrega(&mut pedido, endereco, codigo_cupom)
            .await
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
