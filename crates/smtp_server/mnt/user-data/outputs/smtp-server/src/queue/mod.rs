use std::collections::VecDeque;
use std::sync::Mutex;
use tracing::{error, info, warn};

use crate::{handlers::MailMessage, Config};

/// Status de entrega de um e-mail
#[derive(Debug, Clone, PartialEq)]
pub enum DeliveryStatus {
    Pending,
    Processing,
    Delivered,
    Failed(String),
    Bounced(String),
}

/// Item na fila de entrega
#[derive(Debug, Clone)]
pub struct QueueItem {
    pub mail: MailMessage,
    pub status: DeliveryStatus,
    pub enqueued_at: chrono::DateTime<chrono::Utc>,
    pub next_attempt_at: chrono::DateTime<chrono::Utc>,
}

/// Fila de e-mails pendentes de entrega
pub struct MailQueue {
    queue: Mutex<VecDeque<QueueItem>>,
    config: Config,
}

impl MailQueue {
    pub fn new(config: Config) -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            config,
        }
    }

    /// Adiciona um e-mail à fila de entrega
    pub fn enqueue(&self, mail: MailMessage) {
        let item = QueueItem {
            mail: mail.clone(),
            status: DeliveryStatus::Pending,
            enqueued_at: chrono::Utc::now(),
            next_attempt_at: chrono::Utc::now(),
        };

        let mut queue = self.queue.lock().unwrap();
        queue.push_back(item);
        info!(
            "E-mail {} enfileirado: de={} para={} assunto='{}'",
            mail.id,
            mail.from,
            mail.to.join(", "),
            mail.subject
        );
    }

    /// Retorna o número de itens na fila
    pub fn len(&self) -> usize {
        self.queue.lock().unwrap().len()
    }

    /// Worker assíncrono que processa a fila de entrega continuamente
    pub async fn run_delivery_worker(&self) {
        info!("Worker de entrega iniciado");
        loop {
            let item = {
                let mut queue = self.queue.lock().unwrap();
                // Pega o próximo e-mail pendente cujo tempo de espera passou
                let now = chrono::Utc::now();
                let pos = queue.iter().position(|item| {
                    item.status == DeliveryStatus::Pending && item.next_attempt_at <= now
                });
                pos.and_then(|i| {
                    let mut item = queue.remove(i)?;
                    item.status = DeliveryStatus::Processing;
                    Some(item)
                })
            };

            if let Some(mut item) = item {
                info!(
                    "Processando entrega do e-mail {} (tentativa {})",
                    item.mail.id,
                    item.mail.delivery_attempts + 1
                );

                match self.deliver(&item.mail).await {
                    Ok(_) => {
                        info!("E-mail {} entregue com sucesso", item.mail.id);
                    }
                    Err(e) => {
                        item.mail.delivery_attempts += 1;
                        warn!(
                            "Falha na entrega do e-mail {} (tentativa {}): {}",
                            item.mail.id, item.mail.delivery_attempts, e
                        );

                        // Retry exponencial: 1min, 5min, 30min, 2h, 6h
                        let delays = [1, 5, 30, 120, 360];
                        if let Some(&delay) = delays.get(item.mail.delivery_attempts as usize - 1) {
                            item.status = DeliveryStatus::Pending;
                            item.next_attempt_at = chrono::Utc::now()
                                + chrono::Duration::minutes(delay);
                            let mut queue = self.queue.lock().unwrap();
                            queue.push_back(item);
                        } else {
                            error!(
                                "E-mail {} excedeu máximo de tentativas. Descartando.",
                                item.mail.id
                            );
                        }
                    }
                }
            } else {
                // Fila vazia ou nenhum e-mail pronto — espera 5 segundos
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }

    /// Tenta entregar um e-mail
    async fn deliver(&self, mail: &MailMessage) -> Result<(), String> {
        if let Some(ref relay_host) = self.config.relay_host {
            self.deliver_via_relay(mail, relay_host).await
        } else {
            // Modo simulado: apenas loga o e-mail
            self.deliver_simulated(mail).await
        }
    }

    /// Entrega via relay SMTP (ex: Gmail, SendGrid, MailerSend)
    async fn deliver_via_relay(&self, mail: &MailMessage, relay_host: &str) -> Result<(), String> {
        use lettre::{
            AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
            message::header::ContentType,
            transport::smtp::authentication::Credentials,
        };

        let email_builder = Message::builder()
            .from(
                mail.from
                    .parse()
                    .map_err(|e| format!("Remetente inválido: {}", e))?,
            )
            .subject(&mail.subject);

        // Adiciona destinatários
        let mut builder = email_builder;
        for recipient in &mail.to {
            builder = builder
                .to(recipient.parse().map_err(|e| format!("Destinatário inválido {}: {}", recipient, e))?);
        }

        let email = builder
            .header(ContentType::TEXT_PLAIN)
            .body(mail.body_text.clone())
            .map_err(|e| format!("Erro ao construir e-mail: {}", e))?;

        let mut transport_builder = AsyncSmtpTransport::<Tokio1Executor>::relay(relay_host)
            .map_err(|e| format!("Erro ao configurar relay: {}", e))?
            .port(self.config.relay_port);

        if let (Some(user), Some(pass)) = (&self.config.relay_username, &self.config.relay_password) {
            transport_builder = transport_builder.credentials(Credentials::new(
                user.clone(),
                pass.clone(),
            ));
        }

        let transport = transport_builder.build();

        transport
            .send(email)
            .await
            .map_err(|e| format!("Erro no envio: {}", e))?;

        info!(
            "E-mail {} enviado via relay {} para {}",
            mail.id,
            relay_host,
            mail.to.join(", ")
        );

        Ok(())
    }

    /// Entrega simulada (sem relay configurado)
    async fn deliver_simulated(&self, mail: &MailMessage) -> Result<(), String> {
        // Simula latência de rede
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        info!(
            "=== E-MAIL SIMULADO ===\n\
             ID: {}\n\
             De: {}\n\
             Para: {}\n\
             Assunto: {}\n\
             Corpo: {}\n\
             ======================",
            mail.id,
            mail.from,
            mail.to.join(", "),
            mail.subject,
            mail.body_text.chars().take(200).collect::<String>()
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::MailMessage;

    fn make_config() -> Config {
        Config {
            hostname: "localhost".into(),
            port_smtp: 2525,
            port_smtps: 4465,
            port_submission: 2587,
            tls_cert_path: "".into(),
            tls_key_path: "".into(),
            max_message_size: 10_000_000,
            relay_host: None,
            relay_port: 587,
            relay_username: None,
            relay_password: None,
            require_auth: true,
        }
    }

    #[test]
    fn test_enqueue_and_len() {
        let queue = MailQueue::new(make_config());

        let mut mail = MailMessage::new();
        mail.from = "a@example.com".into();
        mail.to = vec!["b@example.com".into()];

        queue.enqueue(mail);
        assert_eq!(queue.len(), 1);
    }
}
