use async_trait::async_trait;
use uuid::Uuid;
use serde::Serialize;
use crate::models::Pedido;
use crate::domain::errors::DomainResult;

#[derive(Serialize)]
pub struct PedidoComEntrega {
    pub pedido: Pedido,
    pub endereco_entrega: Option<crate::models::EnderecoEntrega>,
}

#[derive(Serialize)]
pub struct PedidoComEntregador {
    pub pedido: Pedido,
    pub entregador_nome: Option<String>,
    pub veiculo: Option<String>,
    pub placa: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PedidoCriado {
    pub uuid: Uuid,
    pub codigo: String,
}

#[async_trait]
pub trait PedidoRepositoryPort: Send + Sync {
    async fn criar(&self, pedido: &Pedido) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Pedido>>;
    async fn buscar_por_codigo(&self, codigo: &str) -> DomainResult<Option<Pedido>>;
    async fn buscar_completo(&self, uuid: Uuid) -> DomainResult<Option<Pedido>>;
    async fn buscar_completos_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Pedido>>;
    async fn buscar_completos_por_usuario(&self, usuario_uuid: Uuid) -> DomainResult<Vec<Pedido>>;
    async fn listar_todos(&self) -> DomainResult<Vec<Pedido>>;
    async fn codigo_existe(&self, codigo: &str) -> DomainResult<bool>;
    async fn atualizar_status(&self, uuid: Uuid, novo_status: &str) -> DomainResult<()>;
    async fn atualizar(&self, pedido: Pedido) -> DomainResult<()>;
    async fn atribuir_entregador(&self, pedido_uuid: Uuid, entregador_uuid: Uuid) -> DomainResult<()>;
    async fn remover_entregador(&self, pedido_uuid: Uuid) -> DomainResult<()>;
    async fn buscar_com_entregador(&self, uuid: Uuid) -> DomainResult<Option<PedidoComEntregador>>;
    async fn buscar_pedido_com_entrega(&self, pedido_uuid: Uuid, loja_uuid: Uuid) -> DomainResult<Option<PedidoComEntrega>>;
}
