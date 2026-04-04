use std::sync::Arc;
use uuid::Uuid;

use crate::models::Entregador;
use crate::repositories::{EntregadorRepository, UsuarioRepository, Repository as _};

#[derive(Clone)]
pub struct EntregadorService {
    repo: Arc<EntregadorRepository>,
    usuario_repo: Arc<UsuarioRepository>,
}

impl EntregadorService {
    pub fn new(repo: Arc<EntregadorRepository>, usuario_repo: Arc<UsuarioRepository>) -> Self {
        Self { repo, usuario_repo }
    }

    pub async fn listar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Entregador>, String> {
        self.repo.buscar_por_loja(loja_uuid).await
    }

    pub async fn listar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Entregador>, String> {
        self.repo.buscar_disponiveis(loja_uuid).await
    }

    pub async fn atualizar(
        &self,
        uuid: Uuid,
        usuario_uuid: Uuid,
        nome: Option<String>,
        celular: Option<String>,
        telefone: Option<String>,
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
                if let Some(tel_val) = &telefone {
                    usuario.telefone = Some(tel_val.clone());
                }
                self.usuario_repo.atualizar(usuario).await?;
            }
        }

        self.repo.atualizar(entregador).await
    }

    pub async fn trocar_email_senha(
        &self,
        usuario_uuid: Uuid,
        novo_email: Option<String>,
        nova_senha: Option<String>,
    ) -> Result<(), String> {
        let mut usuario = self.usuario_repo.buscar_por_uuid(usuario_uuid).await?
            .ok_or("Usuário não encontrado")?;

        if let Some(email) = novo_email {
            usuario.email = email;
        }
        if let Some(senha) = nova_senha {
            usuario.senha_hash = bcrypt::hash(&senha, bcrypt::DEFAULT_COST)
                .map_err(|e| e.to_string())?;
        }

        self.usuario_repo.atualizar(usuario).await
    }

    pub async fn definir_disponivel(&self, uuid: Uuid, disponivel: bool) -> Result<(), String> {
        let mut entregador = self.repo.buscar_por_uuid(uuid).await?
            .ok_or("Entregador não encontrado")?;
        entregador.disponivel = disponivel;
        self.repo.atualizar(entregador).await
    }

    pub async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        self.repo.deletar(uuid).await
    }
}
