use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use crate::domain::enums::EstadoDePedido;
use crate::domain::entities::ParteDeItemPedido;

#[derive(Debug, Clone, PartialEq)]
pub struct AdicionalDeItemDePedido {
    pub uuid: Uuid,
    pub item_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub descricao: String,
    pub preco: Decimal,
}

impl AdicionalDeItemDePedido {
    pub fn new(nome: String, descricao: String, loja_uuid: Uuid, item_uuid: Uuid, preco: Decimal) -> Self {
        Self { uuid: Uuid::new_v4(), item_uuid, loja_uuid, nome, descricao, preco }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemPedido {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub pedido_uuid: Uuid,
    pub quantidade: i32,
    pub observacoes: Option<String>,
    pub adicionais: Vec<AdicionalDeItemDePedido>,
    pub partes: Vec<ParteDeItemPedido>,
}

impl ItemPedido {
    pub fn new(pedido_uuid: Uuid, loja_uuid: Uuid, quantidade: i32, observacoes: Option<String>, partes: Vec<ParteDeItemPedido>) -> Self {
        Self { uuid: Uuid::new_v4(), pedido_uuid, loja_uuid, quantidade, observacoes, adicionais: Vec::new(), partes }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pedido {
    pub uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub entregador_uuid: Option<Uuid>,
    pub status: EstadoDePedido,
    pub total: Decimal,
    pub subtotal: Decimal,
    pub taxa_entrega: Decimal,
    pub desconto: Option<Decimal>,
    pub forma_pagamento: String,
    pub observacoes: Option<String>,
    pub tempo_estimado_min: Option<i32>,
    pub criado_em: DateTime<Utc>,
    pub atualizado_em: DateTime<Utc>,
    pub itens: Vec<ItemPedido>,
    pub partes: Vec<ParteDeItemPedido>,
}

impl Pedido {
    pub fn new(usuario_uuid: Uuid, loja_uuid: Uuid, subtotal: Decimal, taxa_entrega: Decimal, forma_pagamento: String, observacoes: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            uuid: Uuid::new_v4(), usuario_uuid, loja_uuid, entregador_uuid: None,
            status: EstadoDePedido::Criado, total: subtotal + taxa_entrega,
            subtotal, taxa_entrega, desconto: None, forma_pagamento, observacoes,
            tempo_estimado_min: None, criado_em: now, atualizado_em: now,
            itens: Vec::new(), partes: Vec::new(),
        }
    }

    pub fn adicionar_item(&mut self, quantidade: i32, observacoes: Option<String>, partes: Vec<ParteDeItemPedido>) -> Uuid {
        let mut item = ItemPedido::new(self.uuid, self.loja_uuid, quantidade, observacoes, partes);
        let item_uuid = item.uuid;
        for parte in item.partes.iter_mut() { parte.item_uuid = Some(item.uuid); }
        self.itens.push(item);
        item_uuid
    }

    pub fn localizar_item(&mut self, item_uuid: Uuid) -> &mut ItemPedido {
        self.itens.iter_mut().find(|i| i.uuid == item_uuid).expect("Item nao encontrado")
    }
}
