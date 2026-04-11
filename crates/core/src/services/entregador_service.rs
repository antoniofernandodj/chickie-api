use std::sync::Arc;
use uuid::Uuid;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

use crate::models::Entregador;
use crate::ports::{EntregadorRepositoryPort, UsuarioRepositoryPort};

#[derive(Clone)]
pub struct EntregadorService {
    repo: Arc<dyn EntregadorRepositoryPort>,
    usuario_repo: Arc<dyn UsuarioRepositoryPort>,
}

#[allow(dead_code)]
impl EntregadorService {
    pub fn new(repo: Arc<dyn EntregadorRepositoryPort>, usuario_repo: Arc<dyn UsuarioRepositoryPort>) -> Self {
        Self { repo, usuario_repo }
    }

    pub async fn listar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Entregador>, String> {
        self.repo.buscar_por_loja(loja_uuid).await.map_err(|e| e.to_string())
    }

    pub async fn listar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Entregador>, String> {
        self.repo.buscar_disponiveis(loja_uuid).await.map_err(|e| e.to_string())
    }

    pub async fn atualizar(
        &self,
        uuid: Uuid,
        usuario_uuid: Uuid,
        nome: Option<String>,
        celular: Option<String>,
        veiculo: Option<String>,
        placa: Option<String>,
    ) -> Result<(), String> {
        let mut entregador = self.repo.buscar_por_uuid(uuid).await?
            .ok_or("Entregador não encontrado")?;

        entregador.veiculo = veiculo;
        entregador.placa = placa;

        // Atualiza campos da tabela usuario (se fornecidos)
        if let Some(nome_val) = nome {
            if let Some(mut usuario) = self.usuario_repo.buscar_por_uuid(usuario_uuid).await? {
                usuario.nome = nome_val;
                if let Some(cel_val) = &celular {
                    usuario.celular = cel_val.clone();
                }
                self.usuario_repo.atualizar(usuario).await?;
            }
        }

        self.repo.atualizar(entregador).await.map_err(|e| e.to_string())
    }

    pub async fn trocar_email_senha(
        &self,
        usuario_uuid: Uuid,
        novo_email: Option<String>,
        nova_senha: Option<String>,
    ) -> Result<(), String> {
        let mut usuario = self.usuario_repo.buscar_por_uuid(usuario_uuid).await.map_err(|e| e.to_string())?
            .ok_or("Usuário não encontrado")?;

        if let Some(email) = novo_email {
            usuario.email = email;
        }
        if let Some(senha) = nova_senha {
            let salt = SaltString::generate(&mut rand::thread_rng());
            let argon2 = Argon2::default();
            usuario.senha_hash = argon2
                .hash_password(senha.as_bytes(), &salt)
                .map_err(|e| e.to_string())?
                .to_string();
        }

        self.usuario_repo.atualizar(usuario).await.map_err(|e| e.to_string())
    }

    pub async fn definir_disponivel(&self, uuid: Uuid, disponivel: bool) -> Result<(), String> {
        let mut entregador = self.repo.buscar_por_uuid(uuid).await.map_err(|e| e.to_string())?
            .ok_or("Entregador não encontrado")?;
        entregador.disponivel = disponivel;
        self.repo.atualizar(entregador).await.map_err(|e| e.to_string())
    }

    pub async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        self.repo.deletar(uuid).await.map_err(|e| e.to_string())
    }
}
