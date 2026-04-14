use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sqlx::FromRow;
use rust_decimal::Decimal;
use utoipa::ToSchema;

use crate::models::Model;

// --- EnderecoLoja (flat, compatível com FromRow) ---



#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct EnderecoLoja {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub cep: Option<String>,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
}

#[allow(dead_code)]
impl EnderecoLoja {

    pub fn new(
        loja_uuid: Uuid,
        cep: Option<String>,
        logradouro: String,
        numero: String,
        complemento: Option<String>,
        bairro: String,
        cidade: String,
        estado: String,
        latitude: Option<Decimal>,
        longitude: Option<Decimal>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            loja_uuid,
            cep,
            logradouro,
            numero,
            complemento,
            bairro,
            cidade,
            estado,
            latitude,
            longitude,
        }
    }

    pub fn to_proto(&self) -> crate::proto::Endereco {
        crate::proto::Endereco {
            uuid: self.uuid.to_string(),
            cep: self.cep.clone().unwrap_or_default(),
            logradouro: self.logradouro.clone(),
            numero: self.numero.clone(),
            complemento: self.complemento.clone().unwrap_or_default(),
            bairro: self.bairro.clone(),
            cidade: self.cidade.clone(),
            estado: self.estado.clone(),
            latitude: self.latitude.map(|d| d.to_string()).unwrap_or_default(),
            longitude: self.longitude.map(|d| d.to_string()).unwrap_or_default(),
        }
    }
}

// --- EnderecoUsuario (flat, para uso futuro com seu repository) ---

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct EnderecoUsuario {
    pub uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub cep: Option<String>,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
}

impl EnderecoUsuario {
    pub fn new(
        usuario_uuid: Uuid,
        cep: Option<String>,
        logradouro: String,
        numero: String,
        complemento: Option<String>,
        bairro: String,
        cidade: String,
        estado: String,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            usuario_uuid,
            cep,
            logradouro,
            numero,
            complemento,
            bairro,
            cidade,
            estado,
            latitude: None,
            longitude: None,
        }
    }

    pub fn to_proto(&self) -> crate::proto::Endereco {
        crate::proto::Endereco {
            uuid: self.uuid.to_string(),
            cep: self.cep.clone().unwrap_or_default(),
            logradouro: self.logradouro.clone(),
            numero: self.numero.clone(),
            complemento: self.complemento.clone().unwrap_or_default(),
            bairro: self.bairro.clone(),
            cidade: self.cidade.clone(),
            estado: self.estado.clone(),
            latitude: self.latitude.map(|d| d.to_string()).unwrap_or_default(),
            longitude: self.longitude.map(|d| d.to_string()).unwrap_or_default(),
        }
    }
}

// --- EnderecoEntrega (flat, para uso futuro com seu repository) ---

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct EnderecoEntrega {
    pub uuid: Uuid,
    pub loja_uuid: Uuid,
    pub pedido_uuid: Uuid,
    pub cep: Option<String>,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
}

impl EnderecoEntrega {
    pub fn new(
        pedido_uuid: Uuid,
        loja_uuid: Uuid,
        cep: Option<String>,
        logradouro: String,
        numero: String,
        complemento: Option<String>,
        bairro: String,
        cidade: String,
        estado: String,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            pedido_uuid,
            loja_uuid,
            cep,
            logradouro,
            numero,
            complemento,
            bairro,
            cidade,
            estado,
            latitude: None,
            longitude: None,
        }
    }

    pub fn to_proto(&self) -> crate::proto::Endereco {
        crate::proto::Endereco {
            uuid: self.uuid.to_string(),
            cep: self.cep.clone().unwrap_or_default(),
            logradouro: self.logradouro.clone(),
            numero: self.numero.clone(),
            complemento: self.complemento.clone().unwrap_or_default(),
            bairro: self.bairro.clone(),
            cidade: self.cidade.clone(),
            estado: self.estado.clone(),
            latitude: self.latitude.map(|d| d.to_string()).unwrap_or_default(),
            longitude: self.longitude.map(|d| d.to_string()).unwrap_or_default(),
        }
    }
}
    
impl Model for EnderecoLoja {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}

impl Model for EnderecoEntrega {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}

impl Model for EnderecoUsuario {
    fn get_uuid(&self) -> Uuid { self.uuid }
    fn set_uuid(&mut self, uuid: Uuid) { self.uuid = uuid; }
}