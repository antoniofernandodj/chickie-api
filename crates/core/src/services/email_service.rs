use async_trait::async_trait;
use tera::{Context, Tera};
use crate::domain::errors::{DomainError, DomainResult};
use crate::ports::EmailServicePort;

const VERIFICACAO_TEMPLATE: &str =
    include_str!("../templates/verificacao_email.html");

pub struct EmailService {
    api_token: String,
    from_email: String,
    base_url: String,
}

impl EmailService {
    pub fn new() -> Self {
        let api_token = std::env::var("MAILERSEND_API_TOKEN").unwrap_or_default();
        let from_email = std::env::var("EMAIL_FROM").unwrap_or_default();
        let base_url = std::env::var("APP_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:3000".into());

        if api_token.is_empty() {
            tracing::warn!("MAILERSEND_API_TOKEN não configurado — envio de emails desabilitado");
        }

        Self { api_token, from_email, base_url }
    }
}

#[async_trait]
impl EmailServicePort for EmailService {
    async fn enviar_verificacao_cadastro(
        &self,
        email: &str,
        nome: &str,
        token: &str,
    ) -> DomainResult<()> {
        let link = format!("{}/api/auth/confirmar-email?token={}", self.base_url, token);

        let mut ctx = Context::new();
        ctx.insert("nome", nome);
        ctx.insert("link", &link);

        let html = Tera::one_off(VERIFICACAO_TEMPLATE, &ctx, false)
            .map_err(|e| DomainError::Internal(
                format!("Erro ao renderizar template de email: {}", e)
            ))?;

        let body = serde_json::json!({
            "from": { "email": self.from_email },
            "to": [{ "email": email }],
            "subject": "Confirme seu cadastro no Chickie",
            "html": html,
            "text": format!(
                "Olá, {}! Acesse este link para confirmar seu cadastro: {}",
                nome, link
            )
        });

        if self.api_token.is_empty() || self.from_email.is_empty() {
            return Err(DomainError::Internal(
                "Serviço de email não configurado. Defina MAILERSEND_API_TOKEN e EMAIL_FROM.".into()
            ));
        }

        let client = reqwest::Client::new();
        let resp = client
            .post("https://api.mailersend.com/v1/email")
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
            .header("X-Requested-With", "XMLHttpRequest")
            .json(&body)
            .send()
            .await
            .map_err(|e| DomainError::Internal(format!("Erro ao enviar email: {}", e)))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body_text = resp.text().await.unwrap_or_default();
            return Err(DomainError::Internal(
                format!("MailerSend retornou erro {}: {}", status, body_text)
            ));
        }

        tracing::info!("Email de verificação enviado para {}", email);
        Ok(())
    }
}
