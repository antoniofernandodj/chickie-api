use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{EnderecoEntrega, Model}, repositories::Repository};
use crate::ports::EnderecoEntregaRepositoryPort;
use crate::domain::errors::{DomainError, DomainResult};

pub struct EnderecoEntregaRepository { pool: Arc<PgPool> }

impl EnderecoEntregaRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Busca o endereco de entrega vinculado a um pedido especifico
    pub async fn buscar_por_pedido(&self, pedido_uuid: Uuid) -> Result<Option<EnderecoEntrega>, String> {
        sqlx::query_as::<_, EnderecoEntrega>("SELECT * FROM enderecos_entrega WHERE pedido_uuid = $1")
        .bind(pedido_uuid)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    /// Busca enderecos de entrega por loja (util para relatorios/auditoria)
    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<EnderecoEntrega>, String> {
        sqlx::query_as::<_, EnderecoEntrega>("SELECT * FROM enderecos_entrega WHERE loja_uuid = $1 ORDER BY criado_em DESC")
        .bind(loja_uuid)
        .fetch_all(self.pool())
        .await
        .map_err(|e| e.to_string())
    }

    /// Cria um endereco de entrega vinculado a um pedido (uso interno no fluxo de checkout)
    pub async fn criar_para_pedido(
        &self,
        endereco: &EnderecoEntrega,
        pedido_uuid: Uuid,
        loja_uuid: Uuid
    ) -> Result<Uuid, String> {
        sqlx::query("
            INSERT INTO enderecos_entrega (
                uuid, loja_uuid, pedido_uuid, cep, logradouro,
                numero, complemento, bairro, cidade, estado, latitude, longitude
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);
        ")
        .bind(endereco.uuid)
        .bind(loja_uuid)
        .bind(pedido_uuid)
        .bind(&endereco.cep)
        .bind(&endereco.logradouro)
        .bind(&endereco.numero)
        .bind(&endereco.complemento)
        .bind(&endereco.bairro)
        .bind(&endereco.cidade)
        .bind(&endereco.estado)
        .bind(endereco.latitude)
        .bind(endereco.longitude)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;
        Ok(endereco.uuid)
    }
}

#[async_trait::async_trait]
impl Repository<EnderecoEntrega> for EnderecoEntregaRepository {
    fn table_name(&self) -> &'static str { "enderecos_entrega" }
    fn entity_name(&self) -> &'static str { "Endereco de entrega" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(&self, item: &EnderecoEntrega) -> Result<Uuid, String> {
        // Nota: criar diretamente sem pedido_uuid pode nao fazer sentido no dominio
        // Use `criar_para_pedido` para o fluxo normal
        sqlx::query("
            INSERT INTO enderecos_entrega (
                uuid, loja_uuid, pedido_uuid, cep, logradouro,
                numero, complemento, bairro, cidade, estado, latitude, longitude
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);
        ")
        .bind(item.uuid)
        .bind(item.loja_uuid)
        .bind(item.pedido_uuid)
        .bind(&item.cep)
        .bind(&item.logradouro)
        .bind(&item.numero)
        .bind(&item.complemento)
        .bind(&item.bairro)
        .bind(&item.cidade)
        .bind(&item.estado)
        .bind(item.latitude)
        .bind(item.longitude)
        .execute(self.pool())
        .await
        .map_err(|e| e.to_string())?;
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: EnderecoEntrega) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE enderecos_entrega SET loja_uuid = $1, pedido_uuid = $2, cep = $3, logradouro = $4, numero = $5, complemento = $6, bairro = $7, cidade = $8, estado = $9, latitude = $10, longitude = $11
            WHERE uuid = $12
        ")
        .bind(item.loja_uuid)
        .bind(item.pedido_uuid)
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

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<EnderecoEntrega>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}

#[async_trait::async_trait]
impl EnderecoEntregaRepositoryPort for EnderecoEntregaRepository {
    async fn criar(&self, endereco: &EnderecoEntrega) -> DomainResult<Uuid> {
        <Self as Repository<EnderecoEntrega>>::criar(self, endereco).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<EnderecoEntrega>> {
        <Self as Repository<EnderecoEntrega>>::buscar_por_uuid(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_pedido(&self, pedido_uuid: Uuid) -> DomainResult<Option<EnderecoEntrega>> {
        self.buscar_por_pedido(pedido_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<EnderecoEntrega>> {
        self.buscar_por_loja(loja_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn atualizar(&self, endereco: EnderecoEntrega) -> DomainResult<()> {
        <Self as Repository<EnderecoEntrega>>::atualizar(self, endereco).await.map_err(|e| DomainError::Internal(e))
    }
}
