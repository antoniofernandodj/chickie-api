use std::sync::Arc;
use uuid::Uuid;

use crate::models::Funcionario;
use crate::repositories::{FuncionarioRepository, UsuarioRepository, Repository as _};

#[derive(Clone)]
pub struct FuncionarioService {
    repo: Arc<FuncionarioRepository>,
    usuario_repo: Arc<UsuarioRepository>,
}

impl FuncionarioService {
    pub fn new(repo: Arc<FuncionarioRepository>, usuario_repo: Arc<UsuarioRepository>) -> Self {
        Self { repo, usuario_repo }
    }

    pub async fn listar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Funcionario>, String> {
        self.repo.buscar_por_loja(loja_uuid).await
    }

    pub async fn atualizar(
        &self,
        uuid: Uuid,
        usuario_uuid: Uuid,
        nome: Option<String>,
        email: Option<String>,
        senha: Option<String>,
        celular: Option<String>,
        telefone: Option<String>,
        cargo: Option<String>,
        salario: Option<f64>,
        data_admissao: String,
    ) -> Result<(), String> {
        let mut funcionario = self.repo.buscar_por_uuid(uuid).await?
            .ok_or("Funcionário não encontrado")?;

        // Atualiza campos da tabela funcionario
        funcionario.usuario_uuid = usuario_uuid;
        funcionario.cargo = cargo;
        funcionario.salario = salario;
        funcionario.data_admissao = data_admissao;

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
                if let Some(email_val) = &email {
                    usuario.email = email_val.clone();
                }
                if let Some(senha_val) = senha {
                    usuario.senha_hash = bcrypt::hash(&senha_val, bcrypt::DEFAULT_COST)
                        .map_err(|e| e.to_string())?;
                }
                self.usuario_repo.atualizar(usuario).await?;
            }
        }

        self.repo.atualizar(funcionario).await
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

    pub async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        self.repo.deletar(uuid).await
    }
}
