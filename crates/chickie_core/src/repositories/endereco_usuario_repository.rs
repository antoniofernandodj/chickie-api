use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{EnderecoUsuario, Model}, repositories::Repository};
use crate::ports::EnderecoUsuarioRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct EnderecoUsuarioRepository { pool: Arc<PgPool> }

impl EnderecoUsuarioRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Busca todos os enderecos registrados de um usuario
    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<EnderecoUsuario>, String> {
        sqlx::query_as::<_, EnderecoUsuario>("SELECT * FROM enderecos_usuario WHERE usuario_uuid = $1")
        .bind(usuario_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    /// Busca um endereco especifico pelo UUID (helper para validacoes)
    pub async fn buscar_por_uuid_e_usuario(
        &self,
        uuid: Uuid,
        usuario_uuid: Uuid
    ) -> Result<Option<EnderecoUsuario>, String> {
        sqlx::query_as::<_, EnderecoUsuario>("SELECT * FROM enderecos_usuario WHERE uuid = $1 AND usuario_uuid = $2")
        .bind(uuid)
        .bind(usuario_uuid)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<EnderecoUsuario> for EnderecoUsuarioRepository {
    fn table_name(&self) -> &'static str { "enderecos_usuario" }
    fn entity_name(&self) -> &'static str { "Endereco de usuario" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &EnderecoUsuario) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO enderecos_usuario (
                uuid, usuario_uuid, cep, logradouro, numero,
                complemento, bairro, cidade, estado, latitude, longitude
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);
        ")
        .bind(item.uuid).bind(item.usuario_uuid).bind(&item.cep).bind(&item.logradouro)
        .bind(&item.numero).bind(&item.complemento).bind(&item.bairro).bind(&item.cidade)
        .bind(&item.estado).bind(item.latitude).bind(item.longitude)
        .execute(self.pool()).await
        .map_err(|e| e.to_string())?;
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: EnderecoUsuario) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE enderecos_usuario SET usuario_uuid = $1, cep = $2, logradouro = $3, numero = $4, complemento = $5, bairro = $6, cidade = $7, estado = $8, latitude = $9, longitude = $10
            WHERE uuid = $11
        ")
        .bind(item.usuario_uuid)
        .bind(&item.cep)
        .bind(&item.logradouro)
        .bind(&item.numero)
        .bind(&item.complemento)
        .bind(&item.bairro)
        .bind(&item.cidade)
        .bind(&item.estado)
        .bind(item.latitude)
        .bind(item.longitude)
        .bind(uuid)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err(format!("{} não encontrad{}", self.entity_name(), self.entity_gender_suffix()))
        } else {
            Ok(())
        }
    }

    async fn listar_todos_por_loja(&self, _: Uuid) -> Result<Vec<EnderecoUsuario>, String> {
        Err("nao se aplica - enderecos de usuario nao estao vinculados a lojas".into())
    }
}

#[async_trait::async_trait]
impl EnderecoUsuarioRepositoryPort for EnderecoUsuarioRepository {
    async fn criar(&self, endereco: &EnderecoUsuario) -> DomainResult<Uuid> {
        <Self as Repository<EnderecoUsuario>>::criar(self, endereco).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<EnderecoUsuario>> {
        <Self as Repository<EnderecoUsuario>>::buscar_por_uuid(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_por_usuario(&self, usuario_uuid: Uuid) -> DomainResult<Vec<EnderecoUsuario>> {
        self.buscar_por_usuario(usuario_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn atualizar(&self, endereco: EnderecoUsuario) -> DomainResult<()> {
        <Self as Repository<EnderecoUsuario>>::atualizar(self, endereco).await.map_err(|e| DomainError::Internal(e))
    }
    async fn deletar(&self, uuid: Uuid) -> DomainResult<()> {
        <Self as Repository<EnderecoUsuario>>::deletar(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
}
