use std::sync::Arc;
use uuid::Uuid;
use crate::models::{MensagemChat, CreateMensagemRequest};
use crate::ports::{ChatRepositoryPort, ChatPublisherPort};
use crate::domain::errors::DomainResult;

#[derive(Clone)]
pub struct ChatService {
    repo: Arc<dyn ChatRepositoryPort>,
    publisher: Arc<dyn ChatPublisherPort>,
}

impl ChatService {
    pub fn new(repo: Arc<dyn ChatRepositoryPort>, publisher: Arc<dyn ChatPublisherPort>) -> Self {
        Self { repo, publisher }
    }

    pub async fn enviar_mensagem(&self, req: CreateMensagemRequest, remetente_uuid: Uuid) -> DomainResult<MensagemChat> {
        // Persiste no banco
        let msg = self.repo.criar_mensagem(req, remetente_uuid).await?;
        
        // Publica no Redis para tempo real
        let _ = self.publisher.publicar_mensagem(msg.clone()).await;
        
        Ok(msg)
    }

    pub async fn listar_historico_pedido(&self, pedido_uuid: Uuid) -> DomainResult<Vec<MensagemChat>> {
        self.repo.listar_por_pedido(pedido_uuid).await
    }

    pub async fn listar_historico_loja_usuario(&self, loja_uuid: Uuid, usuario_uuid: Uuid) -> DomainResult<Vec<MensagemChat>> {
        self.repo.listar_por_loja_usuario(loja_uuid, usuario_uuid).await
    }

    pub async fn marcar_lida(&self, mensagem_uuid: Uuid) -> DomainResult<()> {
        self.repo.marcar_como_lida(mensagem_uuid).await
    }
}
