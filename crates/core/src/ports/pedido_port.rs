use async_trait::async_trait;
use uuid::Uuid;
use crate::models::Pedido;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait PedidoRepositoryPort: Send + Sync {
    async fn criar(&self, entity: &Pedido) -> DomainResult<Uuid>;
    async fn buscar_por_uuid(&self, uuid: Uuid) -> DomainResult<Option<Pedido>>;
    async fn buscar_completo(&self, uuid: Uuid) -> DomainResult<Option<Pedido>>;
    async fn buscar_completos_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Vec<Pedido>>;
    async fn buscar_completos_por_usuario(&self, usuario_uuid: Uuid) -> DomainResult<Vec<Pedido>>;
    async fn listar_todos(&self) -> DomainResult<Vec<Pedido>>;
    async fn atualizar_status(&self, uuid: Uuid, novo_status: &str) -> DomainResult<()>;
    async fn atualizar(&self, entity: Pedido) -> DomainResult<()>;
}
