use std::sync::Arc;
use crate::services::WhatsAppService;
use crate::domain::errors::DomainResult;

pub struct WhatsAppUsecase {
    whatsapp_service: Arc<WhatsAppService>,
}

impl WhatsAppUsecase {
    pub fn new(whatsapp_service: Arc<WhatsAppService>) -> Self {
        Self { whatsapp_service }
    }

    pub async fn receber_webhook(&self, from: &str, sid: &str, body: &str) -> DomainResult<String> {
        // Formato do Twilio costuma vir como "whatsapp:+5521..."
        let phone = from.replace("whatsapp:", "");
        tracing::info!("WhatsAppUsecase: Processando mensagem de {} (SID={})", phone, sid);
        self.whatsapp_service.processar_mensagem(&phone, sid, body).await
    }
}
