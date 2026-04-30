use uuid::Uuid;
use crate::models::{MensagemChat, CreateMensagemRequest};
use crate::services::ChatService;
use crate::domain::errors::DomainResult;

pub struct ChatUsecase {
    chat_service: ChatService,
}

impl ChatUsecase {
    pub fn new(chat_service: ChatService) -> Self {
        Self { chat_service }
    }

    pub async fn enviar_mensagem(&self, req: CreateMensagemRequest, remetente_uuid: Uuid) -> DomainResult<MensagemChat> {
        self.chat_service.enviar_mensagem(req, remetente_uuid).await
    }

    pub async fn listar_historico_pedido(&self, pedido_uuid: Uuid) -> DomainResult<Vec<MensagemChat>> {
        self.chat_service.listar_historico_pedido(pedido_uuid).await
    }

    pub async fn listar_historico_loja_usuario(&self, loja_uuid: Uuid, usuario_uuid: Uuid) -> DomainResult<Vec<MensagemChat>> {
        self.chat_service.listar_historico_loja_usuario(loja_uuid, usuario_uuid).await
    }

    pub async fn marcar_lida(&self, mensagem_uuid: Uuid) -> DomainResult<()> {
        self.chat_service.marcar_lida(mensagem_uuid).await
    }
}
