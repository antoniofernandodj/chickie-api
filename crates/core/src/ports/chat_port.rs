use async_trait::async_trait;
use uuid::Uuid;
use crate::models::{MensagemChat, CreateMensagemRequest};
use crate::domain::errors::DomainResult;

#[async_trait]
pub trait ChatRepositoryPort: Send + Sync {
    async fn listar_por_pedido(&self, pedido_uuid: Uuid) -> DomainResult<Vec<MensagemChat>>;
    async fn listar_por_loja_usuario(&self, loja_uuid: Uuid, usuario_uuid: Uuid) -> DomainResult<Vec<MensagemChat>>;
    async fn criar_mensagem(&self, req: CreateMensagemRequest, remetente_uuid: Uuid) -> DomainResult<MensagemChat>;
    async fn marcar_como_lida(&self, mensagem_uuid: Uuid) -> DomainResult<()>;
}

#[async_trait]
pub trait ChatPublisherPort: Send + Sync {
    async fn publicar_mensagem(&self, msg: MensagemChat) -> DomainResult<()>;
}
