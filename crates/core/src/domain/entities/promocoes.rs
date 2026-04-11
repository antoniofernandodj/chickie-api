use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use crate::domain::enums::StatusCupom;

#[derive(Debug, Clone, PartialEq)]
pub struct Cupom {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub codigo: String,
    pub descricao: String,
    pub tipo_desconto: String,
    pub valor_desconto: Option<Decimal>,
    pub valor_minimo: Option<Decimal>,
    pub data_validade: String,
    pub limite_uso: Option<i32>,
    pub uso_atual: i32,
    pub status: StatusCupom,
    pub criado_em: DateTime<Utc>,
}

impl Cupom {
    pub fn new(loja_uuid: Uuid, codigo: String, descricao: String, tipo_desconto: String, valor_desconto: Option<Decimal>, valor_minimo: Option<Decimal>, data_validade: String, limite_uso: Option<i32>) -> Self {
        Self { uuid: Uuid::new_v4(), loja_uuid, codigo: codigo.to_uppercase(), descricao, tipo_desconto, valor_desconto, valor_minimo, data_validade, limite_uso, uso_atual: 0, status: StatusCupom::Ativo, criado_em: Utc::now() }
    }

    pub fn calcular_desconto(&self, valor_pedido: Decimal, valor_frete: Decimal) -> Decimal {
        match self.tipo_desconto.as_str() {
            "percentual" => valor_pedido * (self.valor_desconto.unwrap_or(Decimal::ZERO) / Decimal::from(100)),
            "valor_fixo" => self.valor_desconto.unwrap_or(Decimal::ZERO),
            "frete_gratis" => valor_frete,
            _ => Decimal::ZERO,
        }
    }

    pub fn registrar_uso(&mut self) {
        self.uso_atual += 1;
        if let Some(limite) = self.limite_uso { if self.uso_atual >= limite { self.status = StatusCupom::Esgotado; } }
    }

    pub fn ativar(&mut self) { if self.status == StatusCupom::Inativo { self.status = StatusCupom::Ativo; } }
    pub fn desativar(&mut self) { self.status = StatusCupom::Inativo; }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UsoCupom {
    pub uuid: Uuid,
    pub cupom_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub pedido_uuid: Uuid,
    pub valor_desconto: Decimal,
    pub usado_em: String,
}

impl UsoCupom {
    pub fn new(cupom_uuid: Uuid, usuario_uuid: Uuid, pedido_uuid: Uuid, valor_desconto: Decimal) -> Self {
        Self { uuid: Uuid::new_v4(), cupom_uuid, usuario_uuid, pedido_uuid, valor_desconto, usado_em: Utc::now().to_rfc3339() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Promocao {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nome: String,
    pub descricao: String,
    pub tipo_desconto: String,
    pub valor_desconto: Option<Decimal>,
    pub valor_minimo: Option<Decimal>,
    pub data_inicio: String,
    pub data_fim: String,
    pub dias_semana_validos: Option<Vec<i32>>,
    pub tipo_escopo: String,
    pub produto_uuid: Option<Uuid>,
    pub categoria_uuid: Option<Uuid>,
    pub status: StatusCupom,
    pub prioridade: i32,
    pub criado_em: DateTime<Utc>,
}

impl Promocao {
    pub fn new(loja_uuid: Uuid, nome: String, descricao: String, tipo_desconto: String, valor_desconto: Option<Decimal>, valor_minimo: Option<Decimal>, data_inicio: String, data_fim: String, dias_semana_validos: Option<Vec<u8>>, tipo_escopo: String, produto_uuid: Option<Uuid>, categoria_uuid: Option<Uuid>, prioridade: i32) -> Self {
        Self { uuid: Uuid::new_v4(), loja_uuid, nome, descricao, tipo_desconto, valor_desconto, valor_minimo, data_inicio, data_fim, dias_semana_validos: dias_semana_validos.map(|d| d.iter().map(|&n| n as i32).collect()), tipo_escopo, produto_uuid, categoria_uuid, status: StatusCupom::Ativo, prioridade, criado_em: Utc::now() }
    }

    pub fn eh_aplicavel_a_entidade(&self, produto_uuid: Option<Uuid>, categoria_uuid: Option<Uuid>) -> bool {
        match self.tipo_escopo.as_str() {
            "loja" => true,
            "produto" => self.produto_uuid == produto_uuid,
            "categoria" => self.categoria_uuid == categoria_uuid,
            _ => false,
        }
    }

    pub fn calcular_desconto(&self, valor_pedido: Decimal, valor_frete: Decimal) -> Decimal {
        match self.tipo_desconto.as_str() {
            "percentual" => valor_pedido * (self.valor_desconto.unwrap_or(Decimal::ZERO) / Decimal::from(100)),
            "valor_fixo" => self.valor_desconto.unwrap_or(Decimal::ZERO),
            "frete_gratis" => valor_frete,
            _ => Decimal::ZERO,
        }
    }

    pub fn ativar(&mut self) { if self.status == StatusCupom::Inativo { self.status = StatusCupom::Ativo; } }
    pub fn desativar(&mut self) { self.status = StatusCupom::Inativo; }
}
