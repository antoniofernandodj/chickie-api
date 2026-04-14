mod auth;
mod handlers;
mod queue;
mod smtp;
mod tls;

use anyhow::Result;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{
    auth::UserStore,
    queue::MailQueue,
    smtp::SmtpServer,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Carrega .env se existir
    let _ = dotenvy::dotenv();

    // Inicializa o sistema de logs estruturado
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "smtp_server=debug,info".into()))
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .init();

    info!("Iniciando smtp-server v{}", env!("CARGO_PKG_VERSION"));

    // Carrega configurações
    let config = Config::from_env();
    info!("Hostname: {}", config.hostname);
    info!("Porta SMTP: {}", config.port_smtp);
    info!("Porta SMTPS (TLS): {}", config.port_smtps);
    info!("Porta submission: {}", config.port_submission);

    // Inicializa o user store (em produção, usar banco de dados)
    let user_store = Arc::new(UserStore::new_with_defaults());

    // Inicializa a fila de e-mails
    let mail_queue = Arc::new(MailQueue::new(config.clone()));

    // Inicia o worker de entrega de e-mails
    let queue_clone = Arc::clone(&mail_queue);
    tokio::spawn(async move {
        queue_clone.run_delivery_worker().await;
    });

    // Tenta carregar TLS
    let tls_acceptor = match tls::load_tls_config(&config).await {
        Ok(acceptor) => {
            info!("TLS configurado com sucesso");
            Some(Arc::new(acceptor))
        }
        Err(e) => {
            tracing::warn!("TLS não disponível ({}). Rodando sem TLS.", e);
            None
        }
    };

    // Inicia servidores em paralelo
    let mut handles = vec![];

    // Porta SMTP padrão (25) - sem TLS, com STARTTLS opcional
    {
        let addr = format!("0.0.0.0:{}", config.port_smtp);
        let listener = TcpListener::bind(&addr).await?;
        info!("Escutando SMTP em {}", addr);
        let server = SmtpServer::new(
            Arc::clone(&user_store),
            Arc::clone(&mail_queue),
            tls_acceptor.clone(),
            config.clone(),
            false,
        );
        handles.push(tokio::spawn(async move {
            server.run(listener).await;
        }));
    }

    // Porta submission (587) - com STARTTLS
    {
        let addr = format!("0.0.0.0:{}", config.port_submission);
        let listener = TcpListener::bind(&addr).await?;
        info!("Escutando submission em {}", addr);
        let server = SmtpServer::new(
            Arc::clone(&user_store),
            Arc::clone(&mail_queue),
            tls_acceptor.clone(),
            config.clone(),
            false,
        );
        handles.push(tokio::spawn(async move {
            server.run(listener).await;
        }));
    }

    // Porta SMTPS (465) - TLS implícito
    if let Some(ref tls) = tls_acceptor {
        let addr = format!("0.0.0.0:{}", config.port_smtps);
        let listener = TcpListener::bind(&addr).await?;
        info!("Escutando SMTPS (TLS implícito) em {}", addr);
        let server = SmtpServer::new(
            Arc::clone(&user_store),
            Arc::clone(&mail_queue),
            Some(Arc::clone(tls)),
            config.clone(),
            true,
        );
        handles.push(tokio::spawn(async move {
            server.run(listener).await;
        }));
    }

    // Aguarda Ctrl+C
    tokio::signal::ctrl_c().await?;
    info!("Sinal de shutdown recebido. Encerrando...");

    for handle in handles {
        handle.abort();
    }

    Ok(())
}

#[derive(Clone, Debug)]
pub struct Config {
    pub hostname: String,
    pub port_smtp: u16,
    pub port_smtps: u16,
    pub port_submission: u16,
    pub tls_cert_path: String,
    pub tls_key_path: String,
    pub max_message_size: usize,
    pub relay_host: Option<String>,
    pub relay_port: u16,
    pub relay_username: Option<String>,
    pub relay_password: Option<String>,
    pub require_auth: bool,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            hostname: std::env::var("SMTP_HOSTNAME").unwrap_or_else(|_| "localhost".into()),
            port_smtp: std::env::var("SMTP_PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(2525),
            port_smtps: std::env::var("SMTPS_PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(4465),
            port_submission: std::env::var("SMTP_SUBMISSION_PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(2587),
            tls_cert_path: std::env::var("TLS_CERT_PATH").unwrap_or_else(|_| "certs/cert.pem".into()),
            tls_key_path: std::env::var("TLS_KEY_PATH").unwrap_or_else(|_| "certs/key.pem".into()),
            max_message_size: std::env::var("MAX_MESSAGE_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10 * 1024 * 1024), // 10 MB padrão
            relay_host: std::env::var("RELAY_HOST").ok(),
            relay_port: std::env::var("RELAY_PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(587),
            relay_username: std::env::var("RELAY_USERNAME").ok(),
            relay_password: std::env::var("RELAY_PASSWORD").ok(),
            require_auth: std::env::var("REQUIRE_AUTH")
                .map(|v| v == "true" || v == "1")
                .unwrap_or(true),
        }
    }
}
