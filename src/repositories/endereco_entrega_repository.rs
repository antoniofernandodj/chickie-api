use std::sync::Arc;

use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::{models::{EnderecoEntrega, Model}, repositories::Repository};

pub struct EnderecoEntregaRepository { pool: Arc<PgPool> }

impl EnderecoEntregaRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Busca o endereco de entrega vinculado a um pedido especifico
    pub async fn buscar_por_pedido(&self, pedido_uuid: Uuid) -> Result<Option<EnderecoEntrega>, String> {
        sqlx::query_as::<_, EnderecoEntrega>("
            SELECT * FROM enderecos_entrega
            WHERE pedido_uuid = $1;
        ")
        .bind(pedido_uuid)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    /// Busca enderecos de entrega por loja (util para relatorios/auditoria)
    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<EnderecoEntrega>, String> {
        sqlx::query_as::<_, EnderecoEntrega>("
            SELECT * FROM enderecos_entrega
            WHERE loja_uuid = $1
            ORDER BY criado_em DESC;
        ")
        .bind(loja_uuid)
        .fetch_all(&*self.pool)
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
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(endereco.uuid)
    }
}

#[async_trait::async_trait]
impl<'a> Repository<EnderecoEntrega> for EnderecoEntregaRepository {
    fn table_name(&self) -> String { "enderecos_entrega".to_string() }

    async fn buscar_por_uuid(&self, uuid: Uuid) -> Result<Option<EnderecoEntrega>, String> {
        let t = self.table_name();
        let query = format!("SELECT * FROM {} WHERE uuid = $1", t);
        sqlx::query_as::<_, EnderecoEntrega>(&query)
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

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
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(item.uuid)
    }

    async fn atualizar(&self, item: EnderecoEntrega) -> Result<(), String> {
        let uuid = item.get_uuid();
        let result = sqlx::query("
            UPDATE enderecos_entrega
            SET
                loja_uuid = $1, pedido_uuid = $2, cep = $3, logradouro = $4,
                numero = $5, complemento = $6, bairro = $7, cidade = $8,
                estado = $9, latitude = $10, longitude = $11
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
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Endereco de entrega no encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, uuid: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM enderecos_entrega WHERE uuid = $1")
            .bind(uuid)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Endereco de entrega no encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<EnderecoEntrega>, String> {
        sqlx::query_as::<_, EnderecoEntrega>("SELECT * FROM enderecos_entrega;")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<EnderecoEntrega>, String> {
        self.buscar_por_loja(loja_uuid).await
    }
}
