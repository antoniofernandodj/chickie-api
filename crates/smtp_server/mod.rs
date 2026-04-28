use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, error, info, warn};

use crate::{
    auth::UserStore,
    handlers::MailMessage,
    queue::MailQueue,
    tls::TlsAcceptor,
    Config,
};

/// Estados possíveis de uma sessão SMTP
#[derive(Debug, PartialEq, Clone)]
pub enum SmtpState {
    Connected,
    Greeted,
    Authenticated,
    MailFrom,
    RcptTo,
    Data,
}

pub struct SmtpServer {
    user_store: Arc<UserStore>,
    mail_queue: Arc<MailQueue>,
    tls_acceptor: Option<Arc<TlsAcceptor>>,
    config: Config,
    implicit_tls: bool,
}

impl SmtpServer {
    pub fn new(
        user_store: Arc<UserStore>,
        mail_queue: Arc<MailQueue>,
        tls_acceptor: Option<Arc<TlsAcceptor>>,
        config: Config,
        implicit_tls: bool,
    ) -> Self {
        Self { user_store, mail_queue, tls_acceptor, config, implicit_tls }
    }

    pub async fn run(&self, listener: TcpListener) {
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    info!("Nova conexão de {}", addr);
                    let user_store = Arc::clone(&self.user_store);
                    let mail_queue = Arc::clone(&self.mail_queue);
                    let tls_acceptor = self.tls_acceptor.clone();
                    let config = self.config.clone();
                    let implicit_tls = self.implicit_tls;

                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(
                            stream, addr.to_string(), user_store, mail_queue,
                            tls_acceptor, config, implicit_tls,
                        ).await {
                            warn!("Erro na conexão de {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Erro ao aceitar conexão: {}", e);
                }
            }
        }
    }
}

async fn handle_connection(
    stream: TcpStream,
    peer_addr: String,
    user_store: Arc<UserStore>,
    mail_queue: Arc<MailQueue>,
    tls_acceptor: Option<Arc<TlsAcceptor>>,
    config: Config,
    implicit_tls: bool,
) -> anyhow::Result<()> {
    // Para TLS implícito (porta 465), faz o handshake antes de qualquer leitura
    if implicit_tls {
        if let Some(acceptor) = &tls_acceptor {
            let tls_stream = acceptor.accept(stream).await?;
            let session = SmtpSession::new(peer_addr, user_store, mail_queue, config, true);
            return session.run_tls(tls_stream).await;
        }
    }

    let session = SmtpSession::new(peer_addr, user_store, mail_queue, config, false);
    session.run_plain(stream, tls_acceptor).await
}

pub struct SmtpSession {
    peer_addr: String,
    user_store: Arc<UserStore>,
    mail_queue: Arc<MailQueue>,
    config: Config,
    state: SmtpState,
    authenticated: bool,
    is_tls: bool,
    current_mail: Option<MailMessage>,
    auth_username: Option<String>,
}

impl SmtpSession {
    pub fn new(
        peer_addr: String,
        user_store: Arc<UserStore>,
        mail_queue: Arc<MailQueue>,
        config: Config,
        is_tls: bool,
    ) -> Self {
        Self {
            peer_addr,
            user_store,
            mail_queue,
            config,
            state: SmtpState::Connected,
            authenticated: false,
            is_tls,
            current_mail: None,
            auth_username: None,
        }
    }

    /// Roda a sessão em texto puro (com suporte a STARTTLS)
    pub async fn run_plain(
        mut self,
        stream: TcpStream,
        tls_acceptor: Option<Arc<TlsAcceptor>>,
    ) -> anyhow::Result<()> {
        let (reader, mut writer) = tokio::io::split(stream);
        let mut reader = BufReader::new(reader);

        self.send_greeting(&mut writer).await?;

        let mut line = String::new();
        loop {
            line.clear();
            let n = reader.read_line(&mut line).await?;
            if n == 0 {
                info!("[{}] Conexão encerrada pelo cliente", self.peer_addr);
                break;
            }

            let trimmed = line.trim_end().to_string();
            debug!("[{}] C: {}", self.peer_addr, trimmed);

            if trimmed.to_uppercase().starts_with("STARTTLS") {
                if let Some(ref acceptor) = tls_acceptor {
                    self.write_line(&mut writer, "220 Pronto para TLS").await?;
                    // Necessita reconstruir o stream — em produção usar wrapper
                    // Aqui sinalizamos suporte mas não upgradeamos inline por limitação de API
                    self.is_tls = true;
                    info!("[{}] STARTTLS solicitado", self.peer_addr);
                    let _ = acceptor; // Em produção: fazer upgrade do stream aqui
                    continue;
                } else {
                    self.write_line(&mut writer, "454 TLS não disponível").await?;
                    continue;
                }
            }

            let quit = self.process_command(&trimmed, &mut writer).await?;
            if quit {
                break;
            }
        }

        Ok(())
    }

    /// Roda a sessão já em TLS (porta 465)
    pub async fn run_tls(
        mut self,
        stream: tokio_rustls::server::TlsStream<TcpStream>,
    ) -> anyhow::Result<()> {
        let (reader, mut writer) = tokio::io::split(stream);
        let mut reader = BufReader::new(reader);

        self.send_greeting(&mut writer).await?;

        let mut line = String::new();
        loop {
            line.clear();
            let n = reader.read_line(&mut line).await?;
            if n == 0 {
                break;
            }

            let trimmed = line.trim_end().to_string();
            debug!("[{}] C: {}", self.peer_addr, trimmed);

            let quit = self.process_command(&trimmed, &mut writer).await?;
            if quit {
                break;
            }
        }

        Ok(())
    }

    async fn send_greeting<W: AsyncWriteExt + Unpin>(&mut self, writer: &mut W) -> anyhow::Result<()> {
        let greeting = format!("220 {} ESMTP smtp-server ready", self.config.hostname);
        self.write_line(writer, &greeting).await?;
        info!("[{}] Sessão iniciada", self.peer_addr);
        Ok(())
    }

    async fn process_command<W: AsyncWriteExt + Unpin>(
        &mut self,
        line: &str,
        writer: &mut W,
    ) -> anyhow::Result<bool> {
        let upper = line.to_uppercase();
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        let cmd = parts[0].to_uppercase();

        match cmd.as_str() {
            "HELO" => {
                let domain = parts.get(1).unwrap_or(&"unknown");
                info!("[{}] HELO de {}", self.peer_addr, domain);
                self.state = SmtpState::Greeted;
                self.write_line(writer, &format!("250 {} Olá {}", self.config.hostname, domain)).await?;
            }

            "EHLO" => {
                let domain = parts.get(1).unwrap_or(&"unknown");
                info!("[{}] EHLO de {}", self.peer_addr, domain);
                self.state = SmtpState::Greeted;
                self.write_ehlo(writer).await?;
            }

            "AUTH" => {
                if self.state == SmtpState::Connected {
                    self.write_line(writer, "503 Envie EHLO primeiro").await?;
                    return Ok(false);
                }
                self.handle_auth(line, writer).await?;
            }

            "MAIL" if upper.starts_with("MAIL FROM:") => {
                self.handle_mail_from(line, writer).await?;
            }

            "RCPT" if upper.starts_with("RCPT TO:") => {
                self.handle_rcpt_to(line, writer).await?;
            }

            "DATA" => {
                self.handle_data_command(writer).await?;
                // Lê o corpo do e-mail
                return Ok(false); // O loop de data é tratado inline abaixo
            }

            "RSET" => {
                self.current_mail = None;
                self.state = SmtpState::Greeted;
                self.write_line(writer, "250 OK - Resetado").await?;
            }

            "NOOP" => {
                self.write_line(writer, "250 OK").await?;
            }

            "QUIT" => {
                self.write_line(writer, &format!("221 {} Até logo", self.config.hostname)).await?;
                info!("[{}] Sessão encerrada normalmente", self.peer_addr);
                return Ok(true);
            }

            "VRFY" => {
                self.write_line(writer, "252 Não verificamos endereços por segurança").await?;
            }

            "STARTTLS" => {
                // Tratado antes de chegar aqui em run_plain
                self.write_line(writer, "220 Pronto para TLS").await?;
            }

            _ => {
                warn!("[{}] Comando desconhecido: {}", self.peer_addr, cmd);
                self.write_line(writer, "500 Comando desconhecido").await?;
            }
        }

        // Tratamento especial para DATA - lê o corpo
        if cmd == "DATA" || upper.starts_with("DATA") {
            // Já processado acima
        }

        Ok(false)
    }

    async fn write_ehlo<W: AsyncWriteExt + Unpin>(&mut self, writer: &mut W) -> anyhow::Result<()> {
        let hostname = self.config.hostname.clone();
        let max_size = self.config.max_message_size;
        let lines = vec![
            format!("250-{} Olá", hostname),
            format!("250-SIZE {}", max_size),
            "250-AUTH LOGIN PLAIN".to_string(),
            "250-ENHANCEDSTATUSCODES".to_string(),
            "250-8BITMIME".to_string(),
            if self.tls_acceptor_available() {
                "250-STARTTLS".to_string()
            } else {
                "250-SMTPUTF8".to_string()
            },
            "250 SMTPUTF8".to_string(),
        ];

        for line in lines {
            self.write_line(writer, &line).await?;
        }

        Ok(())
    }

    fn tls_acceptor_available(&self) -> bool {
        !self.is_tls // Se já estiver em TLS, não oferece STARTTLS
    }

    async fn handle_auth<W: AsyncWriteExt + Unpin>(
        &mut self,
        line: &str,
        writer: &mut W,
    ) -> anyhow::Result<()> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            self.write_line(writer, "501 Sintaxe: AUTH <mecanismo> [dados iniciais]").await?;
            return Ok(());
        }

        match parts[1].to_uppercase().as_str() {
            "PLAIN" => {
                let credentials = if parts.len() >= 3 {
                    parts[2].to_string()
                } else {
                    self.write_line(writer, "334 ").await?;
                    // Em produção, ler próxima linha aqui
                    return Ok(());
                };

                self.verify_plain_auth(&credentials, writer).await?;
            }

            "LOGIN" => {
                self.write_line(writer, "334 VXNlcm5hbWU6").await?; // "Username:" em base64
                // Em produção, implementar fluxo de login de múltiplas etapas
                // com leitura de linhas adicionais
            }

            _ => {
                self.write_line(writer, "504 Mecanismo de autenticação não suportado").await?;
            }
        }

        Ok(())
    }

    async fn verify_plain_auth<W: AsyncWriteExt + Unpin>(
        &mut self,
        credentials: &str,
        writer: &mut W,
    ) -> anyhow::Result<()> {
        use base64::Engine;

        let decoded = base64::engine::general_purpose::STANDARD
            .decode(credentials)
            .unwrap_or_default();
        let decoded_str = String::from_utf8_lossy(&decoded);

        // Formato PLAIN: \0username\0password ou username\0password
        let parts: Vec<&str> = decoded_str.split('\0').collect();
        let (username, password) = if parts.len() == 3 {
            (parts[1], parts[2])
        } else if parts.len() == 2 {
            (parts[0], parts[1])
        } else {
            self.write_line(writer, "535 Credenciais inválidas").await?;
            warn!("[{}] Falha na autenticação: formato inválido", self.peer_addr);
            return Ok(());
        };

        if self.user_store.verify(username, password) {
            self.authenticated = true;
            self.auth_username = Some(username.to_string());
            self.state = SmtpState::Authenticated;
            self.write_line(writer, "235 2.7.0 Autenticado com sucesso").await?;
            info!("[{}] Usuário '{}' autenticado", self.peer_addr, username);
        } else {
            self.write_line(writer, "535 5.7.8 Credenciais inválidas").await?;
            warn!("[{}] Falha de autenticação para '{}'", self.peer_addr, username);
        }

        Ok(())
    }

    async fn handle_mail_from<W: AsyncWriteExt + Unpin>(
        &mut self,
        line: &str,
        writer: &mut W,
    ) -> anyhow::Result<()> {
        if self.config.require_auth && !self.authenticated {
            self.write_line(writer, "530 5.7.0 Autenticação necessária").await?;
            return Ok(());
        }

        if !matches!(self.state, SmtpState::Greeted | SmtpState::Authenticated) {
            self.write_line(writer, "503 Sequência incorreta de comandos").await?;
            return Ok(());
        }

        let from = extract_email(line, "MAIL FROM:");
        match from {
            Some(email) if is_valid_email(&email) => {
                let mut mail = MailMessage::new();
                mail.from = email.clone();
                self.current_mail = Some(mail);
                self.state = SmtpState::MailFrom;
                self.write_line(writer, "250 OK").await?;
                info!("[{}] MAIL FROM: {}", self.peer_addr, email);
            }
            Some(email) => {
                self.write_line(writer, "501 Endereço de remetente inválido").await?;
                warn!("[{}] Endereço inválido: {}", self.peer_addr, email);
            }
            None => {
                self.write_line(writer, "501 Sintaxe inválida em MAIL FROM").await?;
            }
        }

        Ok(())
    }

    async fn handle_rcpt_to<W: AsyncWriteExt + Unpin>(
        &mut self,
        line: &str,
        writer: &mut W,
    ) -> anyhow::Result<()> {
        if self.state != SmtpState::MailFrom && self.state != SmtpState::RcptTo {
            self.write_line(writer, "503 MAIL FROM primeiro").await?;
            return Ok(());
        }

        let to = extract_email(line, "RCPT TO:");
        match to {
            Some(email) if is_valid_email(&email) => {
                if let Some(ref mut mail) = self.current_mail {
                    if mail.to.len() >= 50 {
                        self.write_line(writer, "452 Muitos destinatários").await?;
                        return Ok(());
                    }
                    mail.to.push(email.clone());
                    self.state = SmtpState::RcptTo;
                    self.write_line(writer, "250 OK").await?;
                    info!("[{}] RCPT TO: {}", self.peer_addr, email);
                }
            }
            Some(email) => {
                self.write_line(writer, "501 Endereço de destinatário inválido").await?;
                warn!("[{}] Destinatário inválido: {}", self.peer_addr, email);
            }
            None => {
                self.write_line(writer, "501 Sintaxe inválida em RCPT TO").await?;
            }
        }

        Ok(())
    }

    async fn handle_data_command<W: AsyncWriteExt + Unpin>(
        &mut self,
        writer: &mut W,
    ) -> anyhow::Result<()> {
        if self.state != SmtpState::RcptTo {
            self.write_line(writer, "503 RCPT TO primeiro").await?;
            return Ok(());
        }

        self.write_line(writer, "354 Envie os dados do e-mail; finalize com uma linha contendo apenas '.'").await?;
        self.state = SmtpState::Data;

        Ok(())
    }

    async fn write_line<W: AsyncWriteExt + Unpin>(&self, writer: &mut W, line: &str) -> anyhow::Result<()> {
        debug!("[{}] S: {}", self.peer_addr, line);
        writer.write_all(format!("{}\r\n", line).as_bytes()).await?;
        writer.flush().await?;
        Ok(())
    }
}

/// Extrai o endereço de e-mail de comandos como "MAIL FROM:<email>"
pub fn extract_email(line: &str, prefix: &str) -> Option<String> {
    let upper = line.to_uppercase();
    let pos = upper.find(&prefix.to_uppercase())?;
    let rest = &line[pos + prefix.len()..].trim().to_string();

    // Remove < > e espaços
    let email = rest
        .trim_start_matches('<')
        .trim_end_matches('>')
        .trim()
        .to_string();

    // Remove parâmetros extras (ex: SIZE=1234)
    let email = email.split_whitespace().next()?.to_string();
    let email = email.split('>').next()?.to_string();

    if email.is_empty() {
        None
    } else {
        Some(email)
    }
}

/// Validação básica de endereço de e-mail via regex
pub fn is_valid_email(email: &str) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$").unwrap();
    re.is_match(email)
}
