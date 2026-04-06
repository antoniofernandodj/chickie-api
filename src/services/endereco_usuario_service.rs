use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::entities::endereco_usuario::Model as EnderecoUsuario;
use crate::repositories::{EnderecoUsuarioRepository, Repository as _};

#[derive(Clone)]
pub struct EnderecoUsuarioService {
    repo: Arc<EnderecoUsuarioRepository>,
}

impl EnderecoUsuarioService {
    pub fn new(repo: Arc<EnderecoUsuarioRepository>) -> Self {
        Self { repo }
    }

    /// Cria um novo endereço para o usuário
    pub async fn criar_endereco(
        &self,
        usuario_uuid: Uuid,
        cep: Option<String>,
        logradouro: String,
        numero: String,
        complemento: Option<String>,
        bairro: String,
        cidade: String,
        estado: String,
        // latitude: Option<f64>,
        // longitude: Option<f64>,
    ) -> Result<EnderecoUsuario, String> {

        let endereco = EnderecoUsuario {
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
        };

        self.repo.criar(&endereco).await?;
        Ok(endereco)
    }

    /// Lista todos os endereços de um usuário
    pub async fn listar_enderecos(&self, usuario_uuid: Uuid) -> Result<Vec<EnderecoUsuario>, String> {
        self.repo.buscar_por_usuario(usuario_uuid).await
    }

    /// Busca um endereço específico, validando que pertence ao usuário (segurança)
    pub async fn buscar_endereco(
        &self,
        uuid: Uuid,
        usuario_uuid: Uuid,
    ) -> Result<Option<EnderecoUsuario>, String> {
        self.repo.buscar_por_uuid_e_usuario(uuid, usuario_uuid).await
    }

    /// Atualiza um endereço existente (apenas se pertencer ao usuário)
    pub async fn atualizar_endereco(
        &self,
        uuid: Uuid,
        usuario_uuid: Uuid,
        cep: Option<String>,
        logradouro: String,
        numero: String,
        complemento: Option<String>,
        bairro: String,
        cidade: String,
        estado: String,
        latitude: Option<Decimal>,
        longitude: Option<Decimal>,
    ) -> Result<EnderecoUsuario, String> {

        // Busca e valida propriedade
        let mut endereco = self.buscar_endereco(uuid, usuario_uuid).await?
            .ok_or("Endereço não encontrado ou não pertence ao usuário")?;

        // Atualiza campos
        endereco.cep = cep;
        endereco.logradouro = logradouro;
        endereco.numero = numero;
        endereco.complemento = complemento;
        endereco.bairro = bairro;
        endereco.cidade = cidade;
        endereco.estado = estado;
        endereco.latitude = latitude;
        endereco.longitude = longitude;

        self.repo.atualizar(endereco.clone()).await?;
        Ok(endereco)
    }

    /// Remove um endereço (apenas se pertencer ao usuário)
    pub async fn deletar_endereco(
        &self,
        uuid: Uuid,
        usuario_uuid: Uuid,
    ) -> Result<(), String> {
        // Valida propriedade antes de deletar
        self.buscar_endereco(uuid, usuario_uuid).await?
            .ok_or("Endereço não encontrado ou não pertence ao usuário")?;

        self.repo.deletar(uuid).await
    }
}
