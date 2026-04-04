use std::sync::Arc;
use uuid::Uuid;

use crate::models::Funcionario;
use crate::repositories::{FuncionarioRepository, Repository as _};

#[derive(Clone)]
pub struct FuncionarioService {
    repo: Arc<FuncionarioRepository>,
}

impl FuncionarioService {
    pub fn new(repo: Arc<FuncionarioRepository>) -> Self {
        Self { repo }
    }

    pub async fn listar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Funcionario>, String> {
        self.repo.buscar_por_loja(loja_uuid).await
    }

    pub async fn atualizar(
        &self,
        uuid: Uuid,
        usuario_uuid: Uuid,
        cargo: Option<String>,
        salario: Option<f64>,
        data_admissao: String,
    ) -> Result<(), String> {
        let mut funcionario = self.repo.buscar_por_uuid(uuid).await?
            .ok_or("Funcionário não encontrado")?;
        funcionario.usuario_uuid = usuario_uuid;
        funcionario.cargo = cargo;
        funcionario.salario = salario;
        funcionario.data_admissao = data_admissao;
        self.repo.atualizar(funcionario).await
    }

    pub async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        self.repo.deletar(uuid).await
    }
}
