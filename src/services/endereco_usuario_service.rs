use std::sync::Arc;
use uuid::Uuid;

use crate::models::EnderecoUsuario;
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
        latitude: Option<f64>,
        longitude: Option<f64>,
    ) -> Result<EnderecoUsuario, String> {
        
        let endereco = EnderecoUsuario::new(
            usuario_uuid,
            cep,
            logradouro,
            numero,
            complemento,
            bairro,
            cidade,
            estado,
            // latitude,
            // longitude,
        );

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
        latitude: Option<f64>,
        longitude: Option<f64>,
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

    /// Define um endereço como "padrão" para o usuário
    /// (Requer campo `padrao: bool` no modelo - se não tiver, ignore este método)
    pub async fn definir_como_padrao(
        &self,
        endereco_uuid: Uuid,
        usuario_uuid: Uuid,
    ) -> Result<(), String> {
        // Se o modelo tiver campo `padrao`, desmarca outros e marca este
        // Exemplo conceitual:
        // 1. UPDATE enderecos_usuario SET padrao = false WHERE usuario_uuid = ?
        // 2. UPDATE enderecos_usuario SET padrao = true WHERE uuid = ? AND usuario_uuid = ?
        unimplemented!("Requer campo 'padrao' no modelo EnderecoUsuario")
    }
}