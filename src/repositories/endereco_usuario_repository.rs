use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{EnderecoUsuario, Model}, repositories::Repository};

pub struct EnderecoUsuarioRepository { pool: Arc<PgPool> }

impl EnderecoUsuarioRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Busca todos os enderecos registrados de um usuario
    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<EnderecoUsuario>, String> {
        sqlx::query_as::<_, EnderecoUsuario>("
            SELECT * FROM enderecos_usuario
            WHERE usuario_uuid = $1;
        ")
        .bind(usuario_uuid)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    /// Busca um endereco especifico pelo UUID (helper para validacoes)
    pub async fn buscar_por_uuid_e_usuario(
        &self,
        uuid: Uuid,
        usuario_uuid: Uuid
    ) -> Result<Option<EnderecoUsuario>, String> {
        sqlx::query_as::<_, EnderecoUsuario>("
            SELECT * FROM enderecos_usuario
            WHERE uuid = $1 AND usuario_uuid = $2;
        ")
        .bind(uuid)
        .bind(usuario_uuid)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl<'a> Repository<EnderecoUsuario> for EnderecoUsuarioRepository {
    fn table_name(&self) -> String { "enderecos_usuario".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<EnderecoUsuario>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = $1", t);
        sqlx::query_as::<_, EnderecoUsuario>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

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
        .execute(&*self.pool).await
        .map_err(|e| e.to_string())?;
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: EnderecoUsuario) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE enderecos_usuario
            SET
                usuario_uuid = $1, cep = $2, logradouro = $3, numero = $4,
                complemento = $5, bairro = $6, cidade = $7, estado = $8,
                latitude = $9, longitude = $10
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
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Endereco de usuario no encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM enderecos_usuario WHERE uuid = $1")
            .bind(uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Endereco de usuario no encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<EnderecoUsuario>, String> {
        sqlx::query_as::<_, EnderecoUsuario>("SELECT * FROM enderecos_usuario;")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, _: Uuid) -> Result<Vec<EnderecoUsuario>, String> {
        Err("nao se aplica - enderecos de usuario nao estao vinculados a lojas".into())
    }
}
