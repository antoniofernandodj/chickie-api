use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Representa um e-mail completo pronto para processamento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailMessage {
    /// ID único do e-mail
    pub id: String,

    /// Endereço do remetente (do MAIL FROM)
    pub from: String,

    /// Lista de destinatários (do RCPT TO)
    pub to: Vec<String>,

    /// Endereços CC extraídos do corpo
    pub cc: Vec<String>,

    /// Endereços BCC extraídos do corpo
    pub bcc: Vec<String>,

    /// Assunto do e-mail
    pub subject: String,

    /// Corpo em texto puro
    pub body_text: String,

    /// Corpo em HTML (se disponível)
    pub body_html: Option<String>,

    /// Cabeçalhos brutos do e-mail
    pub headers: Vec<(String, String)>,

    /// Data/hora de recebimento
    pub received_at: DateTime<Utc>,

    /// Tamanho em bytes
    pub size: usize,

    /// Número de tentativas de entrega
    pub delivery_attempts: u32,
}

impl MailMessage {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            from: String::new(),
            to: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
            subject: String::new(),
            body_text: String::new(),
            body_html: None,
            headers: Vec::new(),
            received_at: Utc::now(),
            size: 0,
            delivery_attempts: 0,
        }
    }

    /// Parseia o corpo bruto do e-mail no formato RFC 2822
    pub fn parse_body(&mut self, raw: &str) {
        self.size = raw.len();

        // Separa cabeçalhos do corpo na linha em branco
        let mut parsing_headers = true;
        let mut body_lines = Vec::new();
        let mut header_buf = String::new();

        for line in raw.lines() {
            if parsing_headers {
                if line.is_empty() {
                    // Processa o último cabeçalho acumulado
                    if !header_buf.is_empty() {
                        self.parse_header(&header_buf);
                        header_buf.clear();
                    }
                    parsing_headers = false;
                    continue;
                }

                // Continuação de linha (RFC 2822: linhas iniciadas por espaço ou tab)
                if line.starts_with(' ') || line.starts_with('\t') {
                    header_buf.push(' ');
                    header_buf.push_str(line.trim());
                } else {
                    if !header_buf.is_empty() {
                        self.parse_header(&header_buf);
                    }
                    header_buf = line.to_string();
                }
            } else {
                body_lines.push(line);
            }
        }

        // Processa o último cabeçalho pendente
        if !header_buf.is_empty() {
            self.parse_header(&header_buf);
        }

        self.body_text = body_lines.join("\n");
    }

    fn parse_header(&mut self, header: &str) {
        if let Some(colon_pos) = header.find(':') {
            let name = header[..colon_pos].trim().to_string();
            let value = header[colon_pos + 1..].trim().to_string();

            match name.to_uppercase().as_str() {
                "SUBJECT" => self.subject = decode_encoded_words(&value),
                "FROM" => {
                    if self.from.is_empty() {
                        self.from = extract_email_from_header(&value);
                    }
                }
                "TO" => {
                    let emails = parse_address_list(&value);
                    self.to.extend(emails);
                }
                "CC" => {
                    let emails = parse_address_list(&value);
                    self.cc.extend(emails);
                }
                "BCC" => {
                    let emails = parse_address_list(&value);
                    self.bcc.extend(emails);
                }
                _ => {}
            }

            self.headers.push((name, value));
        }
    }

    /// Valida os campos obrigatórios do e-mail
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.from.is_empty() {
            errors.push("Remetente não definido".to_string());
        }

        if self.to.is_empty() {
            errors.push("Nenhum destinatário definido".to_string());
        }

        if self.body_text.is_empty() && self.body_html.is_none() {
            errors.push("Corpo do e-mail vazio".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Retorna o cabeçalho Message-ID ou gera um novo
    pub fn message_id(&self) -> String {
        self.headers
            .iter()
            .find(|(k, _)| k.to_uppercase() == "MESSAGE-ID")
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| format!("<{}@smtp-server>", self.id))
    }
}

/// Extrai o endereço de e-mail de uma string como "Nome <email@domain.com>"
fn extract_email_from_header(value: &str) -> String {
    if let Some(start) = value.find('<') {
        if let Some(end) = value.find('>') {
            return value[start + 1..end].trim().to_string();
        }
    }
    value.trim().to_string()
}

/// Parseia uma lista de endereços separados por vírgula
fn parse_address_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|addr| extract_email_from_header(addr.trim()))
        .filter(|e| !e.is_empty())
        .collect()
}

/// Decodifica palavras encoded no formato RFC 2047 (=?charset?encoding?text?=)
fn decode_encoded_words(value: &str) -> String {
    // Implementação simplificada — em produção usar uma biblioteca completa
    if value.contains("=?") {
        // Tenta decodificar o padrão mais comum: =?UTF-8?B?base64?=
        let re = regex::Regex::new(r"=\?[^?]+\?[BbQq]\?([^?]*)\?=").unwrap();
        let result = re.replace_all(value, |caps: &regex::Captures| {
            use base64::Engine;
            let encoded = &caps[1];
            base64::engine::general_purpose::STANDARD
                .decode(encoded)
                .ok()
                .and_then(|bytes| String::from_utf8(bytes).ok())
                .unwrap_or_else(|| caps[0].to_string())
        });
        result.to_string()
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_email() {
        let mut mail = MailMessage::new();
        mail.from = "sender@example.com".to_string();
        mail.to = vec!["recipient@example.com".to_string()];

        let raw = "Subject: Olá Mundo\r\nFrom: sender@example.com\r\nTo: recipient@example.com\r\n\r\nCorpo do e-mail aqui.";
        mail.parse_body(raw);

        assert_eq!(mail.subject, "Olá Mundo");
        assert_eq!(mail.body_text, "Corpo do e-mail aqui.");
    }

    #[test]
    fn test_validate_missing_fields() {
        let mail = MailMessage::new();
        let result = mail.validate();
        assert!(result.is_err());
    }
}
