use std::sync::Arc;

use sqlx::postgres::PgPool;
use sqlx::FromRow;
use uuid::Uuid;
use serde::Serialize;
use utoipa::ToSchema;
use crate::{
    repositories::Repository,
    ports::PedidoRepositoryPort,
    domain::errors::{DomainError, DomainResult},
    models::{
        Model,
        Pedido,
        EstadoDePedido,
        ItemPedido,
    },
};

/// DTO para retorno de pedido com informações do entregador
#[derive(Debug, Clone, FromRow, Serialize, ToSchema)]
pub struct PedidoComEntregador {
    pub uuid: Uuid,
    pub codigo: String,
    pub usuario_uuid: Option<Uuid>,
    pub loja_uuid: Uuid,
    pub entregador_uuid: Option<Uuid>,
    pub status: EstadoDePedido,
    pub total: rust_decimal::Decimal,
    pub subtotal: rust_decimal::Decimal,
    pub taxa_entrega: rust_decimal::Decimal,
    pub desconto: Option<rust_decimal::Decimal>,
    pub forma_pagamento: String,
    pub observacoes: Option<String>,
    pub tempo_estimado_min: Option<i32>,
    pub criado_em: chrono::DateTime<chrono::Utc>,
    pub atualizado_em: chrono::DateTime<chrono::Utc>,
    // Campos do entregador (via LEFT JOIN)
    #[sqlx(skip)]
    pub entregador_nome: String,
    #[sqlx(skip)]
    pub veiculo: String,
    #[sqlx(skip)]
    pub placa: String,
}

pub struct PedidoRepository { pool: Arc<PgPool> }

#[allow(dead_code)]
impl PedidoRepository {
    pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        let stmt = "SELECT * FROM pedidos WHERE usuario_uuid = $1";
        sqlx::query_as::<_, Pedido>(stmt)
            .bind(usuario_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        let stmt = "SELECT * FROM pedidos WHERE loja_uuid = $1";
        sqlx::query_as::<_, Pedido>(stmt)
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_status(&self, status: EstadoDePedido) -> Result<Vec<Pedido>, String> {
        let stmt = "SELECT * FROM pedidos WHERE status = $1";
        sqlx::query_as::<_, Pedido>(stmt)
            .bind(status.to_string())
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn buscar_pendentes(&self, loja_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        let stmt = "SELECT * FROM pedidos WHERE loja_uuid = $1 AND (status = $2 OR status = $3)";
        sqlx::query_as::<_, Pedido>(stmt)
            .bind(loja_uuid)
            .bind(EstadoDePedido::EmPreparo.to_string())
            .bind(EstadoDePedido::Criado.to_string())
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    /// Busca um pedido completo com todos os seus itens, adicionais e partes
    /// Agora usa JSONB em vez de múltiplas tabelas
    pub async fn buscar_completo(
        &self,
        uuid: Uuid,
        loja_uuid: Uuid,
    ) -> Result<Option<Pedido>, String> {
        let mut pedido = match sqlx::query_as::<_, Pedido>(
            "SELECT * FROM pedidos WHERE uuid = $1 AND loja_uuid = $2"
        )
            .bind(uuid)
            .bind(loja_uuid)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())?
        {
            Some(p) => p,
            None => return Ok(None),
        };

        // Parse JSONB para Vec<ItemPedido>
        pedido.itens = Self::parsear_itens_jsonb(&pedido.itens_json)?;
        
        Ok(Some(pedido))
    }

    pub async fn buscar_por_codigo(&self, codigo: &str) -> Result<Option<Pedido>, String> {
        let pedido = match sqlx::query_as::<_, Pedido>(
            "SELECT * FROM pedidos WHERE codigo = $1"
        )
            .bind(codigo)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())?
        {
            Some(p) => p,
            None => return Ok(None),
        };

        let mut pedidos = vec![pedido];
        pedidos = self.hidratar_pedidos(pedidos).await?;
        Ok(Some(pedidos.remove(0)))
    }

    /// Helper para parsear JSONB em Vec<ItemPedido>
    fn parsear_itens_jsonb(
        itens_json: &serde_json::Value
    ) -> Result<Vec<ItemPedido>, String> {
        serde_json::from_value(itens_json.clone())
            .map_err(|e| format!("Erro ao parsear itens JSONB: {}", e))
    }

    /// Mesma logica mas para multiplos pedidos (ex: listar pedidos de uma loja)
    pub async fn buscar_completos_por_loja(
        &self,
        loja_uuid: Uuid,
    ) -> Result<Vec<Pedido>, String> {
        let stmt = "SELECT * FROM pedidos WHERE loja_uuid = $1 ORDER BY criado_em DESC";
        let pedidos = sqlx::query_as::<_, Pedido>(stmt)
            .bind(loja_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        self.hidratar_pedidos(pedidos).await
    }

    pub async fn buscar_completos_por_usuario(
        &self,
        usuario_uuid: Uuid,
    ) -> Result<Vec<Pedido>, String> {
        let stmt = "SELECT * FROM pedidos WHERE usuario_uuid = $1 ORDER BY criado_em DESC";
        let pedidos = sqlx::query_as::<_, Pedido>(stmt)
            .bind(usuario_uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        self.hidratar_pedidos(pedidos).await
    }

    /// Extrai a logica de hidratacao para reuso entre os metodos acima
    /// Agora usa JSONB em vez de queries em múltiplas tabelas
    async fn hidratar_pedidos(
        &self,
        pedidos: Vec<Pedido>,
    ) -> Result<Vec<Pedido>, String> {
        if pedidos.is_empty() {
            return Ok(vec![]);
        }

        // Parse JSONB para cada pedido
        let pedidos_hidratados = pedidos
            .into_iter()
            .map(|mut pedido| {
                pedido.itens = Self::parsear_itens_jsonb(&pedido.itens_json)
                    .unwrap_or_default();
                pedido
            })
            .collect();

        Ok(pedidos_hidratados)
    }

    /// Atribui um entregador a um pedido
    pub async fn atribuir_entregador(
        &self,
        pedido_uuid: Uuid,
        entregador_uuid: Uuid,
    ) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE pedidos SET entregador_uuid = $1 WHERE uuid = $2"
        )
        .bind(entregador_uuid)
        .bind(pedido_uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Pedido não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    /// Remove o entregador de um pedido
    pub async fn remover_entregador(
        &self,
        pedido_uuid: Uuid,
    ) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE pedidos SET entregador_uuid = NULL WHERE uuid = $1"
        )
        .bind(pedido_uuid)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Pedido não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    /// Busca um pedido com dados do entregador (JOIN com entregadores + usuarios)
    pub async fn buscar_com_entregador(
        &self,
        pedido_uuid: Uuid,
    ) -> Result<Option<PedidoComEntregador>, String> {
        let row = sqlx::query_as::<_, PedidoComEntregador>(
            "SELECT
                p.uuid,
                p.codigo,
                p.usuario_uuid,
                p.loja_uuid,
                p.entregador_uuid,
                p.status,
                p.total,
                p.subtotal,
                p.taxa_entrega,
                p.desconto,
                p.forma_pagamento,
                p.observacoes,
                p.tempo_estimado_min,
                p.criado_em,
                p.atualizado_em,
                COALESCE(e.uuid, '00000000-0000-0000-0000-000000000000'::uuid) as entregador_uuid,
                COALESCE(u.nome, '') as entregador_nome,
                COALESCE(e.veiculo, '') as veiculo,
                COALESCE(e.placa, '') as placa
            FROM pedidos p
            LEFT JOIN entregadores e ON p.entregador_uuid = e.uuid
            LEFT JOIN usuarios u ON e.usuario_uuid = u.uuid
            WHERE p.uuid = $1"
        )
        .bind(pedido_uuid)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row)
    }

}

#[async_trait::async_trait]
impl Repository<Pedido> for PedidoRepository {
    fn table_name(&self) -> &'static str { "pedidos" }
    fn entity_name(&self) -> &'static str { "Pedido" }
    fn pool(&self) -> &PgPool { &*self.pool }

    async fn criar(
        &self,
        pedido: &Pedido
    ) -> Result<Uuid, String> {
        // Serializar itens para JSONB
        let itens_json = serde_json::to_value(&pedido.itens)
            .map_err(|e| format!("Erro ao serializar itens para JSON: {}", e))?;

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| e.to_string())?;

        tracing::info!(
            target: "pedido",
            "[REPO] pedido_repo.criar chamado uuid={} loja={} itens={}",
            pedido.uuid, pedido.loja_uuid, pedido.itens.len(),
        );
        
        let stmt = "
            INSERT INTO pedidos (
                uuid, codigo, usuario_uuid, loja_uuid, entregador_uuid, status,
                total, subtotal, taxa_entrega, desconto, forma_pagamento, 
                observacoes, tempo_estimado_min, itens
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14::jsonb)
        ";

        sqlx::query(stmt)
            .bind(&pedido.uuid)
            .bind(&pedido.codigo)
            .bind(&pedido.usuario_uuid)
            .bind(&pedido.loja_uuid)
            .bind(&pedido.entregador_uuid)
            .bind(&pedido.status.to_string())
            .bind(&pedido.total)
            .bind(&pedido.subtotal)
            .bind(&pedido.taxa_entrega)
            .bind(&pedido.desconto)
            .bind(&pedido.forma_pagamento)
            .bind(&pedido.observacoes)
            .bind(&pedido.tempo_estimado_min)
            .bind(&itens_json)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!(target: "pedido", "[REPO] ERRO no INSERT uuid={}: {}", pedido.uuid, e);
                e.to_string()
            })?;

        tracing::info!(target: "pedido", "[REPO] INSERT executado com sucesso uuid={}", pedido.uuid);
        
        tx.commit().await.map_err(|e| {
            tracing::error!(target: "pedido", "[REPO] ERRO no commit uuid={}: {}", pedido.uuid, e);
            e.to_string()
        })?;

        tracing::info!(target: "pedido", "[REPO] transação commitada uuid={}", pedido.uuid);
        Ok(pedido.uuid)
    }

    async fn atualizar(&self, item: Pedido) -> Result<(), String> {
        let uuid = item.get_uuid();
        let stmt = "
            UPDATE pedidos SET status = $1, total = $2, subtotal = $3, taxa_entrega = $4, desconto = $5, forma_pagamento = $6, observacoes = $7, tempo_estimado_min = $8, entregador_uuid = $9
            WHERE uuid = $10
        ";

        let result = sqlx::query(stmt)
            .bind(item.status.to_string())
            .bind(item.total)
            .bind(item.subtotal)
            .bind(item.taxa_entrega)
            .bind(item.desconto)
            .bind(&item.forma_pagamento)
            .bind(&item.observacoes)
            .bind(item.tempo_estimado_min)
            .bind(item.entregador_uuid)
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

    async fn listar_todos_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        let stmt = "SELECT * FROM pedidos WHERE loja_uuid = $1";
        sqlx::query_as::<_, Pedido>(stmt)
            .bind(loja_uuid)
            .fetch_all(self.pool())
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl PedidoRepositoryPort for PedidoRepository {
    async fn criar(&self, pedido: &Pedido) -> DomainResult<Uuid> {
        <Self as Repository<Pedido>>::criar(self, pedido).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Pedido>> {
        <Self as Repository<Pedido>>::buscar_por_uuid(self, uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_por_codigo(&self, codigo: &str) -> DomainResult<Option<Pedido>> {
        PedidoRepository::buscar_por_codigo(self, codigo).await.map_err(DomainError::Internal)
    }
    async fn buscar_completo(&self, uuid: Uuid) -> DomainResult<Option<Pedido>> {
        let pedido = match sqlx::query_as::<_, Pedido>("SELECT * FROM pedidos WHERE uuid = $1")
            .bind(uuid)
            .fetch_optional(&*self.pool)
            .await.map_err(|e| DomainError::Internal(e.to_string()))? {
            Some(p) => p,
            None => return Ok(None),
        };
        // Hydrate with items, parts, adicionais
        let mut pedidos = vec![pedido];
        pedidos = self.hidratar_pedidos(pedidos).await.map_err(|e| DomainError::Internal(e))?;
        Ok(Some(pedidos.remove(0)))
    }
    async fn buscar_completos_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Pedido>> {
        self.buscar_completos_por_loja(loja_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_completos_por_usuario(&self, usuario_uuid: Uuid) -> DomainResult<Vec<Pedido>> {
        self.buscar_completos_por_usuario(usuario_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn listar_todos(&self) -> DomainResult<Vec<Pedido>> {
        <Self as Repository<Pedido>>::listar_todos(self).await.map_err(|e| DomainError::Internal(e))
    }
    async fn codigo_existe(&self, codigo: &str) -> DomainResult<bool> {
        let existe: Option<i64> = sqlx::query_scalar("SELECT 1 FROM pedidos WHERE codigo = $1 LIMIT 1")
            .bind(codigo)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(existe.is_some())
    }
    async fn atualizar_status(&self, uuid: Uuid, novo_status: &str) -> DomainResult<()> {
        sqlx::query("UPDATE pedidos SET status = $1 WHERE uuid = $2")
            .bind(novo_status).bind(uuid)
            .execute(&*self.pool)
            .await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
    async fn atualizar(&self, pedido: Pedido) -> DomainResult<()> {
        <Self as Repository<Pedido>>::atualizar(self, pedido).await.map_err(|e| DomainError::Internal(e))
    }
    async fn atribuir_entregador(&self, pedido_uuid: Uuid, entregador_uuid: Uuid) -> DomainResult<()> {
        self.atribuir_entregador(pedido_uuid, entregador_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn remover_entregador(&self, pedido_uuid: Uuid) -> DomainResult<()> {
        self.remover_entregador(pedido_uuid).await.map_err(|e| DomainError::Internal(e))
    }
    async fn buscar_com_entregador(&self, uuid: Uuid) -> DomainResult<Option<crate::ports::PedidoComEntregador>> {
        let r = PedidoRepository::buscar_com_entregador(self, uuid).await.map_err(|e| DomainError::Internal(e))?;
        Ok(r.map(|r| {
            let pedido = Pedido {
                uuid: r.uuid, codigo: r.codigo, usuario_uuid: r.usuario_uuid, loja_uuid: r.loja_uuid,
                entregador_uuid: r.entregador_uuid, status: r.status, total: r.total,
                subtotal: r.subtotal, taxa_entrega: r.taxa_entrega, desconto: r.desconto,
                forma_pagamento: r.forma_pagamento, observacoes: r.observacoes,
                tempo_estimado_min: r.tempo_estimado_min, criado_em: r.criado_em,
                atualizado_em: r.atualizado_em, itens_json: serde_json::Value::Array(vec![]), itens: vec![],
            };
            crate::ports::PedidoComEntregador {
                pedido,
                entregador_nome: Some(r.entregador_nome),
                veiculo: Some(r.veiculo),
                placa: Some(r.placa),
            }
        }))
    }
    async fn buscar_pedido_com_entrega(&self, pedido_uuid: Uuid, _loja_uuid: Uuid) -> DomainResult<Option<crate::ports::PedidoComEntrega>> {
        self.buscar_pedido_com_entrega(pedido_uuid, _loja_uuid).await.map_err(|e| DomainError::Internal(e.to_string()))
    }
}
