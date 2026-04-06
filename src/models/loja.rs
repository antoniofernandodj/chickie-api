use uuid::Uuid;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::{Utc, NaiveTime};
use rust_decimal::Decimal;
use utoipa::ToSchema;

use crate::models::Model;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Loja {
    pub uuid: Uuid,
    pub nome: String,
    pub slug: String,
    pub descricao: Option<String>,
    pub email: String,
    pub telefone: Option<String>,
    pub ativa: bool,
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
    pub horario_abertura: Option<NaiveTime>,
    pub horario_fechamento: Option<NaiveTime>,
    pub dias_funcionamento: Option<Vec<i32>>,  // [0,1,2,3,4,5,6] → Domingo..Sábado
    pub tempo_preparo_min: Option<i32>,
    pub taxa_entrega: Option<Decimal>,
    pub valor_minimo_pedido: Option<Decimal>,
    pub raio_entrega_km: Option<Decimal>,
    pub criado_por: Option<Uuid>,  // Admin que criou a loja
    pub criado_em: chrono::DateTime<chrono::Utc>,
    pub atualizado_em: chrono::DateTime<chrono::Utc>,
}

impl Loja {
    pub fn new(
        nome: String,
        slug: String,
        email: String,
        descricao: Option<String>,
        telefone: Option<String>,
        horario_abertura: Option<NaiveTime>,
        horario_fechamento: Option<NaiveTime>,
        dias_funcionamento: Option<Vec<i32>>,
        tempo_preparo_min: Option<i32>,
        taxa_entrega: Option<Decimal>,
        valor_minimo_pedido: Option<Decimal>,
        raio_entrega_km: Option<Decimal>,
        criado_por: Option<Uuid>,
    ) -> Self {

        Self {
            uuid: Uuid::new_v4(),
            nome,
            slug,
            descricao,
            email,
            telefone,
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
            criado_em: Utc::now(),
            atualizado_em: Utc::now(),
        }
    }
}


impl Model for Loja {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
