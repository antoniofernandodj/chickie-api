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
    pub celular: Option<String>,
    pub ativa: bool,  // Loja operacional (owner pode desativar)
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

    // Soft delete fields
    #[serde(default)]
    pub marcado_para_remocao: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub deletado: bool,

    // Flag para bloqueio administrativo (ex: inadimplência)
    // Diferente de 'ativa': este só admin do sistema pode alterar
    #[serde(default = "default_true")]
    pub ativo: bool,

    // Block flag — explicitly blocks store from operating
    #[serde(default)]
    pub bloqueado: bool,

    pub criado_em: chrono::DateTime<chrono::Utc>,
    pub atualizado_em: chrono::DateTime<chrono::Utc>,
}

fn default_true() -> bool { true }

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
            marcado_para_remocao: None,
            deletado: false,
            ativo: true,
            bloqueado: false,
            criado_em: Utc::now(),
            atualizado_em: Utc::now(),
        }
    }

    /// Verifica se a loja está marcada para remoção
    pub fn esta_marcada_para_remocao(&self) -> bool {
        self.marcado_para_remocao.is_some() && !self.deletado
    }

    /// Verifica se a loja está permanentemente deletada
    pub fn esta_deletada(&self) -> bool {
        self.deletado
    }

    /// Verifica se a loja está operacional (ativa + não deletada + não marcada para remoção + ativo + não bloqueado)
    pub fn esta_operacional(&self) -> bool {
        self.ativa && !self.deletado && self.marcado_para_remocao.is_none() && self.ativo && !self.bloqueado
    }

    /// Verifica se a loja está bloqueada
    pub fn esta_bloqueada(&self) -> bool {
        self.bloqueado
    }

    pub fn to_proto(&self) -> crate::proto::Loja {
        crate::proto::Loja {
            uuid: self.uuid.to_string(),
            nome: self.nome.clone(),
            slug: self.slug.clone(),
            descricao: self.descricao.clone().unwrap_or_default(),
            email: self.email.clone(),
            celular: self.celular.clone().unwrap_or_default(),
            ativa: self.ativa,
            logo_url: self.logo_url.clone().unwrap_or_default(),
            banner_url: self.banner_url.clone().unwrap_or_default(),
        }
    }
}


impl Model for Loja {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}
