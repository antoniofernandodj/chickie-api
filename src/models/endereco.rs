use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sqlx::FromRow;

use crate::models::Model;

// --- EnderecoLoja (flat, compatível com FromRow) ---



#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
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
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

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
        latitude: Option<f64>,
        longitude: Option<f64>,
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
}

// --- EnderecoUsuario (flat, para uso futuro com seu repository) ---

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
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
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
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
}

// --- EnderecoEntrega (flat, para uso futuro com seu repository) ---

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
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
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
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