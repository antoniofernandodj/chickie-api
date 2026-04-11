use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use crate::domain::enums::TipoCalculoPedido;
use crate::domain::entities::AdicionalDeItemDePedido;

#[derive(Debug, Clone, PartialEq)]
pub struct Produto {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub categoria_uuid: Uuid,
    pub nome: String,
    pub descricao: Option<String>,
    pub preco: Decimal,
    pub imagem_url: Option<String>,
    pub disponivel: bool,
    pub tempo_preparo_min: Option<i32>,
    pub destaque: bool,
    pub criado_em: DateTime<Utc>,
    pub atualizado_em: DateTime<Utc>,
}

impl Produto {
    pub fn new(nome: String, descricao: Option<String>, preco: Decimal, categoria_uuid: Uuid, loja_uuid: Uuid, tempo_preparo_min: Option<i32>) -> Self {
        let now = Utc::now();
        Self { uuid: Uuid::new_v4(), loja_uuid, categoria_uuid, nome, descricao, preco, imagem_url: None, disponivel: false, tempo_preparo_min, destaque: false, criado_em: now, atualizado_em: now }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParteDeItemPedido {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub item_uuid: Option<Uuid>,
    pub produto_nome: String,
    pub produto_uuid: Uuid,
    pub preco_unitario: Decimal,
    pub posicao: i32,
    pub adicionais: Vec<AdicionalDeItemDePedido>,
}

impl ParteDeItemPedido {
    pub fn new(produto: &Produto, posicao: i32) -> Self {
        Self { uuid: Uuid::new_v4(), loja_uuid: produto.loja_uuid, item_uuid: None, produto_uuid: produto.uuid, produto_nome: produto.nome.clone(), preco_unitario: produto.preco, posicao, adicionais: vec![] }
    }

    pub fn set_item_uuid(&mut self, item_uuid: Uuid) { self.item_uuid = Some(item_uuid); }

    pub fn adicionar_adicional(&mut self, adicional: &Adicional) -> Result<Uuid, String> {
        if self.loja_uuid != adicional.loja_uuid { return Err("Adicional pertence a outra loja".to_string()); }
        let novo = AdicionalDeItemDePedido::new(adicional.nome.clone(), adicional.descricao.clone(), adicional.loja_uuid, self.uuid, adicional.preco);
        let uuid = novo.uuid;
        self.adicionais.push(novo);
        Ok(uuid)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfiguracaoDePedidosLoja {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub max_partes: i32,
    pub tipo_calculo: TipoCalculoPedido,
    pub criado_em: DateTime<Utc>,
    pub atualizado_em: DateTime<Utc>,
}

impl ConfiguracaoDePedidosLoja {
    pub fn new(loja_uuid: Uuid, max_partes: i32, tipo_calculo: TipoCalculoPedido) -> Result<Self, String> {
        if max_partes < 1 { return Err("max_partes deve ser >= 1".into()); }
        let now = Utc::now();
        Ok(Self { uuid: Uuid::new_v4(), loja_uuid, max_partes, tipo_calculo, criado_em: now, atualizado_em: now })
    }
}

pub fn calcular_preco_por_partes(sabores: &[ParteDeItemPedido], tipo: &TipoCalculoPedido) -> Decimal {
    if sabores.is_empty() { return Decimal::ZERO; }
    match tipo {
        TipoCalculoPedido::MediaPonderada => {
            let soma: Decimal = sabores.iter().map(|s| s.preco_unitario).sum();
            soma / Decimal::from(sabores.len())
        }
        TipoCalculoPedido::MaisCaro => sabores.iter().map(|s| s.preco_unitario).fold(Decimal::MIN, Decimal::max),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Adicional {
    pub uuid: Uuid,
    pub nome: String,
    pub loja_uuid: Uuid,
    pub disponivel: bool,
    pub descricao: String,
    pub preco: Decimal,
    pub criado_em: DateTime<Utc>,
}

impl Adicional {
    pub fn new(nome: String, loja_uuid: Uuid, descricao: String, preco: Decimal) -> Self {
        Self { nome, loja_uuid, disponivel: false, descricao, preco, uuid: Uuid::new_v4(), criado_em: Utc::now() }
    }
}
