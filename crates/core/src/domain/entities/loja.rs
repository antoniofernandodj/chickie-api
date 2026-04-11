use uuid::Uuid;
use chrono::{DateTime, NaiveTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq)]
pub struct Loja {
    pub uuid: Uuid,
    pub nome: String,
    pub slug: String,
    pub descricao: Option<String>,
    pub email: String,
    pub celular: Option<String>,
    pub ativa: bool,
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
    pub horario_abertura: Option<NaiveTime>,
    pub horario_fechamento: Option<NaiveTime>,
    pub dias_funcionamento: Option<Vec<i32>>,
    pub tempo_preparo_min: Option<i32>,
    pub taxa_entrega: Option<Decimal>,
    pub valor_minimo_pedido: Option<Decimal>,
    pub raio_entrega_km: Option<Decimal>,
    pub criado_por: Option<Uuid>,
    pub criado_em: DateTime<Utc>,
    pub atualizado_em: DateTime<Utc>,
}

impl Loja {
    pub fn new(
        nome: String,
        slug: String,
        email: String,
        descricao: Option<String>,
        celular: Option<String>,
        horario_abertura: Option<NaiveTime>,
        horario_fechamento: Option<NaiveTime>,
        dias_funcionamento: Option<Vec<i32>>,
        tempo_preparo_min: Option<i32>,
        taxa_entrega: Option<Decimal>,
        valor_minimo_pedido: Option<Decimal>,
        raio_entrega_km: Option<Decimal>,
        criado_por: Option<Uuid>,
    ) -> Self {
        let now = Utc::now();
        Self {
            uuid: Uuid::new_v4(),
            nome,
            slug,
            descricao,
            email,
            celular,
            ativa: true,
            logo_url: None,
            banner_url: None,
            horario_abertura,
            horario_fechamento,
            dias_funcionamento,
            tempo_preparo_min,
            taxa_entrega,
            valor_minimo_pedido,
            raio_entrega_km,
            criado_por,
            criado_em: now,
            atualizado_em: now,
        }
    }
}
