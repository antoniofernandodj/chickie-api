use async_trait::async_trait;
use uuid::Uuid;
use crate::models::ConfiguracaoDePedidosLoja;
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait ConfiguracaoPedidosLojaRepositoryPort: Send + Sync {
    async fn salvar(&self, config: &ConfiguracaoDePedidosLoja) -> DomainResult<()>;
    async fn buscar_por_loja(&self, loja_uuid: Uuid) -> DomainResult<Option<ConfiguracaoDePedidosLoja>>;
}
