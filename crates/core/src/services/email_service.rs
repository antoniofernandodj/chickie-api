//! Serviço de envio de email via SMTP (implementação atual).
//!
//! # Implementação atual: SMTP
//!
//! Utiliza a crate `lettre` com STARTTLS (porta 587).
//! Variáveis de ambiente necessárias:
//!
//! ```env
//! SMTP_NAME=Chickie
//! SMTP_SERVER=smtp.seudominio.com
//! SMTP_PORT=587
//! SMTP_USER=no-reply@seudominio.com
//! SMTP_PASS=sua_senha
//! FRONTEND_BASE_URL=https://app.seudominio.com
//! ```
use async_trait::async_trait;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{header::ContentType, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
};
use tera::{Context, Tera};

use crate::domain::errors::{DomainError, DomainResult};
use crate::ports::EmailServicePort;

const VERIFICACAO_TEMPLATE: &str =
    include_str!("../templates/verificacao_email.html");

/// Implementação SMTP do `EmailServicePort`.
///
/// Lê configuração de variáveis de ambiente no momento da construção.
/// Se `SMTP_SERVER` não estiver definido, loga um warning e falha apenas
/// no momento do envio — não impede o startup da aplicação.
pub struct EmailService {
    smtp_name: String,
    smtp_server: String,
    smtp_port: u16,
    smtp_user: String,
    smtp_pass: String,
    frontend_base_url: String,
}

impl EmailService {
    pub fn new() -> Self {
        let smtp_server = std::env::var("SMTP_SERVER").unwrap_or_default();

        if smtp_server.is_empty() {
            tracing::warn!("SMTP_SERVER não configurado — envio de emails desabilitado");
        }

        Self {
            smtp_name:  std::env::var("SMTP_NAME").unwrap_or_else(|_| "Chickie".into()),
            smtp_server,
            smtp_port:  std::env::var("SMTP_PORT")
                            .unwrap_or_else(|_| "587".into())
                            .parse()
                            .unwrap_or(587),
            smtp_user:          std::env::var("SMTP_USER").unwrap_or_default(),
            smtp_pass:          std::env::var("SMTP_PASS").unwrap_or_default(),
            frontend_base_url:  std::env::var("FRONTEND_BASE_URL")
                                    .unwrap_or_else(|_| "http://localhost:5173".into()),
        }
    }
}

#[async_trait]
impl EmailServicePort for EmailService {
    /// Renderiza o template HTML de verificação de cadastro via Tera e envia
    /// por SMTP com STARTTLS. O email contém versão texto puro como fallback.
    async fn enviar_verificacao_cadastro(
        &self,
        email: &str,
        nome: &str,
        token: &str,
    ) -> DomainResult<()> {
        if self.smtp_server.is_empty() || self.smtp_user.is_empty() {
            return Err(DomainError::Internal(
                "Serviço de email não configurado. Defina SMTP_SERVER, SMTP_USER e SMTP_PASS.".into()
            ));
        }

        let link = format!("{}/auth/confirmar-email?token={}", self.frontend_base_url, token);

        let mut ctx = Context::new();
        ctx.insert("nome", nome);
        ctx.insert("link", &link);

        let html = Tera::one_off(VERIFICACAO_TEMPLATE, &ctx, false)
            .map_err(|e| DomainError::Internal(
                format!("Erro ao renderizar template de email: {}", e)
            ))?;

        let text = format!(
            "Olá, {}! Acesse este link para confirmar seu cadastro: {}",
            nome, link
        );

        let from = format!("{} <{}>", self.smtp_name, self.smtp_user)
            .parse()
            .map_err(|e| DomainError::Internal(format!("Remetente inválido: {}", e)))?;

        let to = email
            .parse()
            .map_err(|_| DomainError::Internal(format!("Email destinatário inválido: {}", email)))?;

        let message = Message::builder()
            .from(from)
            .to(to)
            .subject("Confirme seu cadastro no Chickie")
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(text)
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(html)
                    )
            )
            .map_err(|e| DomainError::Internal(format!("Erro ao construir email: {}", e)))?;

        let creds = Credentials::new(self.smtp_user.clone(), self.smtp_pass.clone());

        let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.smtp_server)
            .map_err(|e| DomainError::Internal(format!("Erro ao criar transporte SMTP: {}", e)))?
            .port(self.smtp_port)
            .credentials(creds)
            .build();

        transport
            .send(message)
            .await
            .map_err(|e| DomainError::Internal(format!("Erro ao enviar email: {}", e)))?;

        tracing::info!("Email de verificação enviado para {}", email);
        Ok(())
    }
}
