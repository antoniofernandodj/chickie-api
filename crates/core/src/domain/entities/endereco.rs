use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq)]
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

impl EnderecoLoja {
    pub fn new(loja_uuid: Uuid, cep: Option<String>, logradouro: String, numero: String, complemento: Option<String>, bairro: String, cidade: String, estado: String, latitude: Option<Decimal>, longitude: Option<Decimal>) -> Self {
        Self { uuid: Uuid::new_v4(), loja_uuid, cep, logradouro, numero, complemento, bairro, cidade, estado, latitude, longitude }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    pub fn new(usuario_uuid: Uuid, cep: Option<String>, logradouro: String, numero: String, complemento: Option<String>, bairro: String, cidade: String, estado: String) -> Self {
        Self { uuid: Uuid::new_v4(), usuario_uuid, cep, logradouro, numero, complemento, bairro, cidade, estado, latitude: None, longitude: None }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    pub fn new(pedido_uuid: Uuid, loja_uuid: Uuid, cep: Option<String>, logradouro: String, numero: String, complemento: Option<String>, bairro: String, cidade: String, estado: String) -> Self {
        Self { uuid: Uuid::new_v4(), pedido_uuid, loja_uuid, cep, logradouro, numero, complemento, bairro, cidade, estado, latitude: None, longitude: None }
    }
}
