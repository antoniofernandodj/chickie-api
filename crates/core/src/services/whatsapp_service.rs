use std::sync::Arc;
use uuid::Uuid;
use crate::ports::{
    WhatsAppRepositoryPort, 
    WhatsAppConversationPort, 
    WhatsAppIdentityType,
    UsuarioRepositoryPort,
    PedidoRepositoryPort,
};
use crate::domain::errors::DomainResult;

pub struct WhatsAppService {
    whatsapp_repo: Arc<dyn WhatsAppRepositoryPort>,
    conv_repo: Arc<dyn WhatsAppConversationPort>,
    _usuario_repo: Arc<dyn UsuarioRepositoryPort>,
    pedido_repo: Arc<dyn PedidoRepositoryPort>,
}

impl WhatsAppService {
    pub fn new(
        whatsapp_repo: Arc<dyn WhatsAppRepositoryPort>,
        conv_repo: Arc<dyn WhatsAppConversationPort>,
        usuario_repo: Arc<dyn UsuarioRepositoryPort>,
        pedido_repo: Arc<dyn PedidoRepositoryPort>,
    ) -> Self {
        Self {
            whatsapp_repo,
            conv_repo,
            _usuario_repo: usuario_repo,
            pedido_repo,
        }
    }

    pub async fn resolver_identidade(&self, phone_number: &str, message_body: &str) -> DomainResult<(WhatsAppIdentityType, Option<Uuid>)> {
        // 1. Verificar vínculo permanente
        if let Some(binding) = self.whatsapp_repo.buscar_binding_por_phone(phone_number).await? {
            if binding.verified {
                return Ok((WhatsAppIdentityType::Authenticated, Some(binding.user_id)));
            }
        }

        // 2. Verificar estado de conversa (Redis)
        if let Some(state) = self.conv_repo.get_state(phone_number).await? {
            return Ok((state.identity_type, state.identifier));
        }

        // 3. Verificar se há um código de pedido na mensagem (ex: "status ABC123")
        for codigo in self.extrair_codigo_pedido(message_body) {
            if let Some(pedido) = self.pedido_repo.buscar_por_codigo(&codigo).await? {
                // Se o pedido tem usuário, resolvemos como Authenticated (embora sem vínculo verificado ainda?)
                // Na verdade, se o usuário fornece o código de um pedido dele, podemos tratar como Guest daquele pedido
                // ou Authenticated se o usuario_uuid bater.
                if let Some(user_id) = pedido.usuario_uuid {
                     return Ok((WhatsAppIdentityType::Authenticated, Some(user_id)));
                } else {
                     return Ok((WhatsAppIdentityType::Guest, Some(pedido.uuid)));
                }
            }
        }

        Ok((WhatsAppIdentityType::Anonymous, None))
    }

    fn extrair_codigo_pedido(&self, body: &str) -> Vec<String> {
        // Regex simples para código de 6 caracteres alfanuméricos
        let body_upper = body.to_uppercase();
        // Procurar por padrão de 6 caracteres que pareçam um código
        // Exemplo: "status ABC123"
        let mut result = Vec::<String>::new();
        for word in body_upper.split_whitespace() {
            if word.len() == 6 && word.chars().all(|c| c.is_alphanumeric()) {
                result.push(word.to_string());
            }
        }
        result
    }

    pub async fn processar_mensagem(&self, phone_number: &str, message_sid: &str, body: &str) -> DomainResult<String> {
        // Idempotência
        if self.whatsapp_repo.ja_processada(message_sid).await? {
            return Ok("".to_string()); // Já processada
        }

        let (identity, identifier) = self
            .resolver_identidade(
                phone_number,
                body
            )
            .await?;
        
        // Registrar processamento
        self.whatsapp_repo.registrar_mensagem_processada(message_sid).await?;

        // Lógica de comando simples (MVP)
        let response = match body.to_lowercase().trim() {
            "ajuda" | "help" | "/start" => self.get_help_message(),
            b if b.contains("status") || b.contains("pedido") => self.handle_status_command(phone_number, identity, identifier, body).await?,
            _ => "Olá! Digite 'ajuda' para ver o que posso fazer por você.".to_string(),
        };

        Ok(response)
    }

    fn get_help_message(&self) -> String {
        "Comandos disponíveis:\n\
        - status [código]: Ver o status do seu pedido\n\
        - lista: Ver o catálogo de produtos\n\
        - ajuda: Ver esta mensagem"
            .to_string()
    }

    async fn handle_status_command(&self, _phone_number: &str, identity: WhatsAppIdentityType, identifier: Option<Uuid>, body: &str) -> DomainResult<String> {
        let codigos = self.extrair_codigo_pedido(body);
        
        let mut pedido_encontrado = None;
        for c in &codigos {
            if let Some(p) = self.pedido_repo.buscar_por_codigo(c).await? {
                pedido_encontrado = Some(p);
                break;
            }
        }

        let pedido = if let Some(p) = pedido_encontrado {
            Some(p)
        } else if let Some(id) = identifier {
            // Se for Guest, identifier é o UUID do pedido
            if matches!(identity, WhatsAppIdentityType::Guest) {
                self.pedido_repo.buscar_por_uuid(id).await?
            } else {
                // Se for Authenticated, buscar último pedido do usuário
                let pedidos = self.pedido_repo.listar_por_usuario(id).await?;
                pedidos.first().cloned()
            }
        } else {
            None
        };

        match pedido {
            Some(p) => {
                let status_str = p.status.as_str().replace("_", " ");
                Ok(format!("Seu pedido #{} está no status: *{}*.", p.codigo, status_str))
            },
            None => Ok("Não encontrei nenhum pedido. Por favor, digite 'status' seguido do código de 6 dígitos.".to_string())
        }
    }
}
